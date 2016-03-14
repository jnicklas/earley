use parse::Value;
use grammar::{Token, RuleName};

pub trait Production<N, O> where N: RuleName {
    fn get_name(&self) -> N;

    fn get_tokens(&self) -> &[Token<N>];

    fn perform(&self, result: Vec<Value<O>>) -> O;
}

impl<'a, N, O> PartialEq for &'a Production<N, O> where N: RuleName {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.get_tokens() == other.get_tokens()
    }
}

impl<'a, N, O> Eq for &'a Production<N, O> where N: RuleName {}
