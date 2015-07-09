use grammar::{Rule};
use token::Token;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Item<'a> {
    pub rule: &'a Rule,
    pub start: usize,
    pub next: usize,
}

impl<'a> Item<'a> {
    pub fn new(rule: &Rule, start: usize) -> Item {
        Item { rule: rule, next: 0, start: start }
    }

    pub fn advance(&self) -> Item<'a> {
        Item { rule: self.rule, next: self.next + 1, start: self.start }
    }

    pub fn next_token(&self) -> Option<&Token> {
        self.rule.tokens.get(self.next)
    }

    pub fn is_complete(&self) -> bool {
        self.next >= self.rule.tokens.len()
    }
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tokens: Vec<String> = self.rule.tokens.iter().map(|t| t.name()).collect();
        if self.next < tokens.len() {
            tokens.insert(self.next, "*".to_string());
        } else {
            tokens.push("*".to_string());
        }
        format!("{} -> {} ({})", self.rule.name, tokens.connect(" "), self.start).fmt(f)
    }
}

