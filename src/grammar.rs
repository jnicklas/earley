use token::{Token, Terminal, NonTerminal};
use item_table::ItemTable;
use std::collections::BTreeMap;
use std::cell::Cell;

#[derive(Debug)]
pub struct Grammar {
    starting_rule: &'static str,
    rules: BTreeMap<&'static str, Rule>,
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
            grammar.get_or_insert_rule_mut(production.name).add_production(production);
        }

        mark_nullable(&mut grammar);

        grammar
    }

    pub fn get_starting_rule_name(&self) -> &'static str {
        self.starting_rule
    }

    pub fn get_rule(&self, name: &'static str) -> Option<&Rule> {
        self.rules.get(name)
    }

    fn get_or_insert_rule_mut(&mut self, name: &'static str) -> &mut Rule {
        self.rules.entry(name).or_insert_with(|| Rule::new(name))
    }

    pub fn productions_for_starting_rule(&self) -> &[Production] {
        &self.rules[self.starting_rule].productions
    }

    pub fn productions_for<'a, 'b>(&'a self, name: &'b str) -> &'a [Production] {
        &self.rules[name].productions
    }

    pub fn build_table<'a>(&'a self, input: &'a str) -> ItemTable<'a> {
        ItemTable::build(self, input)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production {
    name: &'static str,
    tokens: Vec<Token>
}

impl Production {
    pub fn new(name: &'static str, tokens: &[Token]) -> Self {
        Production { name: name, tokens: tokens.iter().cloned().collect() }
    }

    pub fn get_name(&self) -> &'static str {
        &self.name
    }

    pub fn get_tokens(&self) -> &[Token] {
        &self.tokens
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule {
    name: &'static str,
    productions: Vec<Production>,
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

    pub fn get_name(&self) -> &'static str {
        &self.name
    }

    pub fn get_productions(&self) -> &[Production] {
        &self.productions
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
