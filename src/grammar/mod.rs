pub mod production;
pub mod rule;
#[macro_use]
pub mod macros;

use group_by::GroupByExt;
use token::{Terminal, NonTerminal};
use item_table::ItemTable;
use std::collections::BTreeMap;
use parse::parse;
pub use grammar::production::{Production};
pub use grammar::rule::Rule;

pub struct Grammar<T> {
    starting_rule: &'static str,
    rules: BTreeMap<&'static str, Rule<T>>,
}

impl<T> Grammar<T> {
    pub fn new(productions: Vec<Box<Production<T>>>) -> Grammar<T> {
        let first_rule_name = {
            productions.get(0).expect("grammar must have at least one rule").get_name()
        };

        let mut rules = productions
            .into_iter()
            .group_by(|p| p.get_name())
            .map(|(name, productions)| (name, Rule::new(name, productions)))
            .collect();

        mark_nullable(&mut rules);

        Grammar {
            starting_rule: first_rule_name,
            rules: rules
        }
    }

    pub fn get_starting_rule_name(&self) -> &'static str {
        self.starting_rule
    }

    pub fn get_rule(&self, name: &str) -> Option<&Rule<T>> {
        self.rules.get(name)
    }

    pub fn productions_for_starting_rule(&self) -> &[Box<Production<T>>] {
        &self.rules[self.starting_rule].get_productions()
    }

    pub fn productions_for(&self, name: &str) -> &[Box<Production<T>>] {
        &self.rules[name].get_productions()
    }

    pub fn build_table<'a>(&'a self, input: &'a str) -> ItemTable<'a, T> where T: 'a {
        ItemTable::build(self, input)
    }

    pub fn parse<'a>(&'a self, input: &'a str) -> Option<T> where T: 'a {
        let table = self.build_table(input);
        parse(&table)
    }
}

fn mark_nullable<T>(rules: &mut BTreeMap<&'static str, Rule<T>>) {
    loop {
        let mut found_nullable_rule = false;
        for (_, rule) in rules.iter() {
            if rule.is_nullable() {
                continue;
            } else {
                let nullable = rule.get_productions().iter().any(|production| {
                    production.get_tokens().len() == 0 || production.get_tokens().iter().all(|token| {
                        match token {
                            &Terminal(_) => false,
                            &NonTerminal(name) => rules[name].is_nullable()
                        }
                    })
                });
                if nullable {
                    rule.mark_as_nullable();
                    found_nullable_rule = true;
                }
            }
        }
        if !found_nullable_rule { break }
    }
}
