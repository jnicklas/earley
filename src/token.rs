use std::fmt;
use grammar::Lexeme;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token<K> where K: Lexeme {
    Terminal(&'static str),
    NonTerminal(K),
}

pub use Token::Terminal;
pub use Token::NonTerminal;

impl<K> fmt::Display for Token<K> where K: Lexeme + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Terminal(str) => write!(f, "'{}'", str),
            NonTerminal(str) => str.fmt(f)
        }
    }
}
