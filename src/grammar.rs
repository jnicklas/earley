use token::{Token, Terminal, NonTerminal};
use item_table::ItemTable;
use unicode_segmentation::UnicodeSegmentation;
use std::collections::BTreeMap;
use std::cell::Cell;

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
            let rule = grammar.get_rule_mut(production.name).add_production(production);
        }

        mark_nullable(&mut grammar);

        grammar
    }

    pub fn get_rule(&mut self, name: &'static str) -> &Rule {
        self.rules.entry(name).or_insert_with(|| Rule::new(name))
    }

    pub fn get_rule_mut(&mut self, name: &'static str) -> &mut Rule {
        self.rules.entry(name).or_insert_with(|| Rule::new(name))
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
    nullable: Cell<bool>,
}

impl Rule {
    pub fn new(name: &'static str) -> Rule {
        Rule { name: name, productions: Vec::new(), nullable: Cell::new(false) }
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable.get()
    }

    fn add_production(&mut self, production: Production) {
        self.productions.push(production);
    }
}

fn mark_nullable(grammar: &mut Grammar) {
    loop {
        let mut found_nullable_rule = false;
        for (_, rule) in &grammar.rules {
            if rule.is_nullable() {
                continue;
            } else {
                let nullable = rule.productions.iter().any(|production| {
                    production.tokens.len() == 0 || production.tokens.iter().all(|token| {
                        match token {
                            &Terminal(_) => false,
                            &NonTerminal(name) => grammar.rules[name].is_nullable()
                        }
                    })
                });
                if nullable {
                    rule.nullable.set(true);
                    found_nullable_rule = true;
                }
            }
        }
        if !found_nullable_rule { break }
    }
}
