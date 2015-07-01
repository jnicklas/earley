#[derive(Debug, PartialEq, Eq, Clone, Copy)]

pub enum Token {
    Terminal(&'static str),
    NonTerminal(&'static str),
}

pub use Token::Terminal;
pub use Token::NonTerminal;

impl Token {
    pub fn name(&self) -> String {
        match *self {
            Terminal(str) => format!("'{}'", str),
            NonTerminal(str) => str.into()
        }
    }
}

