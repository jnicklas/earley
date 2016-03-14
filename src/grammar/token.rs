use std::fmt;
use grammar::RuleName;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token<N> where N: RuleName {
    Terminal(&'static str),
    NonTerminal(N),
}

pub use Token::Terminal;
pub use Token::NonTerminal;

impl<N> fmt::Display for Token<N> where N: RuleName + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Terminal(str) => write!(f, "'{}'", str),
            NonTerminal(str) => str.fmt(f)
        }
    }
}
