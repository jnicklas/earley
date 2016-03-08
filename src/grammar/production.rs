use token::Token;
use parse::Value;

pub trait Production<T> {
    fn get_name(&self) -> &'static str;

    fn get_tokens(&self) -> &[Token];

    fn perform<'a>(&self, result: Vec<Value<'a, T>>) -> T;
}

impl<'a, T> PartialEq for &'a Production<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && self.get_tokens() == other.get_tokens()
    }
}

impl<'a, T> Eq for &'a Production<T> {}
