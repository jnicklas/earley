use grammar::{Production, Lexeme};
use token::Token;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation<'a, I> where I: 'a {
    Scan(&'a I),
    Predict,
    Complete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item<'a, T, I> where T: Lexeme + PartialEq<&'a I>, I: 'a {
    operation: Operation<'a, I>,
    production: &'a Production<T>,
    start: usize,
    next: usize,
}

impl<'a, T, I> Item<'a, T, I> where T: Lexeme + PartialEq<&'a I>, I: 'a {
    pub fn predict(production: &'a Production<T>, start: usize) -> Item<'a, T, I> {
        Item { operation: Operation::Predict, production: production, next: 0, start: start }
    }

    pub fn scan(&self, value: &'a I) -> Item<'a, T, I> {
        Item { operation: Operation::Scan(value), production: self.production, next: self.next + 1, start: self.start }
    }

    pub fn complete(&self) -> Item<'a, T, I> {
        Item { operation: Operation::Complete, production: self.production, next: self.next + 1, start: self.start }
    }
    pub fn next_token(&self) -> Option<&Token<T>> {
        self.production.get_tokens().get(self.next)
    }

    pub fn is_complete(&self) -> bool {
        self.next >= self.production.get_tokens().len()
    }

    pub fn has_same_production(&self, other: &Item<T, I>) -> bool {
        self.production == other.production
    }

    pub fn get_operation(&self) -> Operation<I> {
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

    pub fn get_tokens(&self) -> &[Token<T>] {
        self.production.get_tokens()
    }
}

impl<'a, T, I> fmt::Display for Item<'a, T, I> where T: fmt::Display + Lexeme + PartialEq<&'a I>, I: 'a + fmt::Display {
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
