use token::{Token, Terminal, NonTerminal};
use item_table::ItemTable;
use std::collections::BTreeMap;
use std::cell::Cell;

pub trait Lexeme: Eq + Ord + Clone + 'static {
    fn matches(&self, other: Self) -> bool;
}

#[derive(Debug)]
pub struct Grammar<T> where T: Lexeme {
    starting_rule: &'static str,
    rules: BTreeMap<&'static str, Rule<T>>,
}

impl<T> Grammar<T> where T: Lexeme {
    pub fn new(productions: Vec<Production<T>>) -> Grammar<T> {
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

    pub fn get_starting_rule(&self) -> &'static str {
        self.starting_rule
    }

    pub fn get_rule(&self, name: &'static str) -> Option<&Rule<T>> {
        self.rules.get(name)
    }

    fn get_or_insert_rule_mut(&mut self, name: &'static str) -> &mut Rule<T> {
        self.rules.entry(name).or_insert_with(|| Rule::new(name))
    }

    pub fn productions_for_starting_rule(&self) -> &[Production<T>] {
        &self.rules[&self.starting_rule].productions
    }

    pub fn productions_for(&self, name: &'static str) -> &[Production<T>] {
        &self.rules[name].productions
    }

    pub fn build_table<'a, A, B, I>(&'a self, input: A) -> ItemTable<'a, T, I> where T: PartialEq<&'a I>, A: IntoIterator<Item=&'a I, IntoIter=B>, B: ExactSizeIterator {
        ItemTable::build(self, input)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production<T> where T: Lexeme {
    name: &'static str,
    tokens: Vec<Token<T>>
}

impl<T> Production<T> where T: Lexeme {
    pub fn new(name: &'static str, tokens: &[Token<T>]) -> Self {
        Production { name: name, tokens: tokens.iter().cloned().collect() }
    }

    pub fn get_name(&self) -> &'static str {
        &self.name
    }

    pub fn get_tokens(&self) -> &[Token<T>] {
        &self.tokens
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rule<T> where T: Lexeme {
    name: &'static str,
    productions: Vec<Production<T>>,
    nullable: Cell<bool>,
}

impl<T> Rule<T> where T: Lexeme {
    pub fn new(name: &'static str) -> Rule<T> {
        Rule { name: name, productions: Vec::new(), nullable: Cell::new(false) }
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable.get()
    }

    fn add_production(&mut self, production: Production<T>) {
        self.productions.push(production);
    }

    pub fn get_name(&self) -> &'static str {
        &self.name
    }

    pub fn get_productions(&self) -> &[Production<T>] {
        &self.productions
    }
}

fn mark_nullable<T>(grammar: &mut Grammar<T>) where T: Lexeme {
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
