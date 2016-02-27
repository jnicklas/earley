use token::{Token, Terminal, NonTerminal};
use item_table::ItemTable;
use unicode_segmentation::UnicodeSegmentation;
use nullability::mark_nullable;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Grammar {
    pub starting_rule: &'static str,
    pub rules: BTreeMap<&'static str, Rule>,
}

impl Grammar {
    pub fn new(productions: Vec<Production>) -> Grammar {
        let mut grammar = {
            let first_production = productions.get(0).expect("grammar must have at least one rule");

            Grammar {
                starting_rule: first_production.name,
                rules: BTreeMap::new(),
            }
        };

        for production in productions {
            let rule = grammar.rules.entry(production.name).or_insert_with(|| Rule::new(production.name));
            rule.add_production(production);
        }

        grammar
    }

    pub fn productions_for_starting_rule(&self) -> &[Production] {
        &self.rules[self.starting_rule].productions
    }

    pub fn productions_for<'a, 'b>(&'a self, name: &'b str) -> &'a [Production] {
        &self.rules[name].productions
    }

    pub fn build_table<'a>(&'a self, input: &'a str) -> ItemTable<'a> {
        let mut s = ItemTable::new(self, input.len());

        let chars = UnicodeSegmentation::graphemes(input, true).chain(Some("\0").into_iter());

        for (char_index, current_char) in chars.enumerate() {
            debug!("-----> {} matching {}", char_index, current_char);
            let mut item_index = 0;
            while item_index < s.table[char_index].len() {
                let item = s.table[char_index][item_index];
                let next_token = item.next_token();
                debug!("[{}, {}] :: {} || {:?}", char_index, item_index, item, next_token);
                match next_token {
                    Some(&NonTerminal(token)) => s.predict(char_index, token),
                    Some(&Terminal(token)) => s.scan(item, char_index, current_char, token),
                    None => s.complete(item, char_index),
                }
                item_index += 1;
            }
        }

        return s;
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production {
    pub name: &'static str,
    pub tokens: Vec<Token>
}

impl Production {
    pub fn new(name: &'static str, tokens: &[Token]) -> Self {
        Production { name: name, tokens: tokens.iter().cloned().collect() }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    pub name: &'static str,
    pub productions: Vec<Production>,
    nullable: bool,
}

impl Rule {
    pub fn new(name: &'static str) -> Rule {
        Rule { name: name, productions: Vec::new(), nullable: false }
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable
    }

    fn add_production(&mut self, production: Production) {
        self.productions.push(production);
    }
}
