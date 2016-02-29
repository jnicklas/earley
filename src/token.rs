use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token<T> {
    Terminal(T),
    NonTerminal(&'static str),
}

pub use Token::Terminal;
pub use Token::NonTerminal;

impl<T> fmt::Display for Token<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Terminal(str) => write!(f, "'{}'", str),
            NonTerminal(str) => str.fmt(f)
        }
    }
}
