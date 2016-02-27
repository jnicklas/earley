use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Terminal(&'static str),
    NonTerminal(&'static str),
}

pub use Token::Terminal;
pub use Token::NonTerminal;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Terminal(str) => write!(f, "'{}'", str),
            NonTerminal(str) => str.fmt(f)
        }
    }
}
