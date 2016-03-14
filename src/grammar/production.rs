use parse::Value;
use grammar::{Token, RuleName};

pub trait Production<T, N> where N: RuleName {
    fn get_name(&self) -> N;

    fn get_tokens(&self) -> &[Token<N>];

    fn perform<'a>(&self, result: Vec<Value<'a, T>>) -> T;
}

impl<'a, T, N> PartialEq for &'a Production<T, N> where N: RuleName {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.get_tokens() == other.get_tokens()
    }
}

impl<'a, T, N> Eq for &'a Production<T, N> where N: RuleName {}
