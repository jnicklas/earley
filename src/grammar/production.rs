use parse::Value;
use grammar::{Token, RuleName};

pub trait Production<T, K> where K: RuleName {
    fn get_name(&self) -> K;

    fn get_tokens(&self) -> &[Token<K>];

    fn perform<'a>(&self, result: Vec<Value<'a, T>>) -> T;
}

impl<'a, T, K> PartialEq for &'a Production<T, K> where K: RuleName {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.get_tokens() == other.get_tokens()
    }
}

impl<'a, T, K> Eq for &'a Production<T, K> where K: RuleName {}
