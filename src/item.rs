use std::fmt;
use parse::Value;
use grammar::{Production, Token, RuleName};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation<'a> {
    Scan(&'a str),
    Predict,
    Complete,
}

#[derive(Eq)]
pub struct Item<'a, T, K> where T: 'a, K: RuleName {
    operation: Operation<'a>,
    production: &'a Production<T, K>,
    start: usize,
    next: usize,
}

impl<'a, T, K> PartialEq for Item<'a, T, K> where T: 'a, K: RuleName {
    fn eq(&self, other: &Self) -> bool {
        self.operation == other.operation
            && self.production == other.production
            && self.start == other.start
            && self.next == other.next
    }
}

impl<'a, T, K> Clone for Item<'a, T, K> where T: 'a, K: RuleName {
    fn clone(&self) -> Self {
        Item {
            operation: self.operation.clone(),
            production: self.production,
            start: self.start,
            next: self.next,
        }
    }
}

impl<'a, T, K> Item<'a, T, K> where T: 'a, K: RuleName {
    pub fn predict(production: &'a Production<T, K>, start: usize) -> Item<'a, T, K> {
        Item { operation: Operation::Predict, production: production, next: 0, start: start }
    }

    pub fn scan(&self, value: &'a str) -> Item<'a, T, K> {
        Item { operation: Operation::Scan(value), production: self.production, next: self.next + 1, start: self.start }
    }

    pub fn complete(&self) -> Item<'a, T, K> {
        Item { operation: Operation::Complete, production: self.production, next: self.next + 1, start: self.start }
    }
    pub fn next_token(&self) -> Option<&Token<K>> {
        self.production.get_tokens().get(self.next)
    }

    pub fn is_complete(&self) -> bool {
        self.next >= self.production.get_tokens().len()
    }

    pub fn has_same_production(&self, other: &'a Item<'a, T, K>) -> bool {
        self.production == other.production
    }

    pub fn get_operation(&self) -> Operation {
        self.operation
    }

    pub fn get_scanned_value(&self) -> Option<&'a str> {
        match self.operation {
            Operation::Scan(value) => Some(value),
            _ => None
        }
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_name(&self) -> K {
        self.production.get_name()
    }

    pub fn get_tokens(&self) -> &[Token<K>] {
        self.production.get_tokens()
    }

    pub fn perform(&self, result: Vec<Value<'a, T>>) -> T {
        self.production.perform(result)
    }
}

impl<'a, T, K> fmt::Display for Item<'a, T, K> where T: 'a, K: RuleName + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tokens: Vec<String> = self.production.get_tokens().iter().map(|t| t.to_string()).collect();
        if self.next < tokens.len() {
            tokens.insert(self.next, "*".to_string());
        } else {
            tokens.push("*".to_string());
        }
        format!("{} -> {} ({})", self.production.get_name(), tokens.join(" "), self.start).fmt(f)
    }
}
