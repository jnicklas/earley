use grammar::{Production};
use token::Token;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation<'a> {
    Scan(&'a str),
    Predict,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item<'a> {
    operation: Operation<'a>,
    production: &'a Production,
    start: usize,
    next: usize,
}

impl<'a> Item<'a> {
    pub fn predict(production: &'a Production, start: usize) -> Item<'a> {
        Item { operation: Operation::Predict, production: production, next: 0, start: start }
    }

    pub fn scan(&self, value: &'a str) -> Item<'a> {
        Item { operation: Operation::Scan(value), production: self.production, next: self.next + 1, start: self.start }
    }

    pub fn complete(&self) -> Item<'a> {
        Item { operation: Operation::Complete, production: self.production, next: self.next + 1, start: self.start }
    }
    pub fn next_token(&self) -> Option<&Token> {
        self.production.get_tokens().get(self.next)
    }

    pub fn is_complete(&self) -> bool {
        self.next >= self.production.get_tokens().len()
    }

    pub fn has_same_production(&self, other: &Item) -> bool {
        self.production == other.production
    }

    pub fn get_operation(&self) -> Operation {
        self.operation
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_next(&self) -> usize {
        self.next
    }

    pub fn get_name(&self) -> &'static str {
        self.production.get_name()
    }

    pub fn get_tokens(&self) -> &[Token] {
        self.production.get_tokens()
    }
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tokens: Vec<String> = self.production.get_tokens().iter().map(|t| t.name()).collect();
        if self.next < tokens.len() {
            tokens.insert(self.next, "*".to_string());
        } else {
            tokens.push("*".to_string());
        }
        format!("{} -> {} ({})", self.production.get_name(), tokens.join(" "), self.start).fmt(f)
    }
}
