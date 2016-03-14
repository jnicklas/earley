pub mod production;
pub mod token;
pub mod rule;
pub mod rule_name;
#[macro_use]
pub mod macros;

use group_by::GroupByExt;
use item_table::ItemTable;
use std::collections::BTreeMap;
use parse::parse;
pub use grammar::production::{Production};
pub use grammar::rule::Rule;
pub use grammar::rule_name::RuleName;
pub use grammar::token::{Token, Terminal, NonTerminal};

type RuleMap<T, K> = BTreeMap<K, Rule<T, K>>;

pub struct Grammar<T, K> where K: RuleName {
    starting_rule: K,
    rules: RuleMap<T, K>,
}

impl<T, K> Grammar<T, K> where K: RuleName {
    pub fn new(productions: Vec<Box<Production<T, K>>>) -> Grammar<T, K> {
        let first_rule_name = {
            productions.get(0).expect("grammar must have at least one rule").get_name()
        };

        let mut rules: RuleMap<T, K> = productions
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

    pub fn get_starting_rule_name(&self) -> K {
        self.starting_rule
    }

    pub fn get_rule(&self, name: K) -> Option<&Rule<T, K>> {
        self.rules.get(&name)
    }

    pub fn productions_for_starting_rule(&self) -> &[Box<Production<T, K>>] {
        &self.rules[&self.starting_rule].get_productions()
    }

    pub fn productions_for(&self, name: K) -> &[Box<Production<T, K>>] {
        &self.rules[&name].get_productions()
    }

    pub fn build_table<'a>(&'a self, input: &'a str) -> ItemTable<'a, T, K> where T: 'a {
        ItemTable::build(self, input)
    }

    pub fn parse<'a>(&'a self, input: &'a str) -> Option<T> where T: 'a {
        let table = self.build_table(input);
        parse(&table)
    }
}

fn mark_nullable<T, K>(rules: &mut RuleMap<T, K>) where K: RuleName {
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
                            &NonTerminal(name) => rules[&name].is_nullable()
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
