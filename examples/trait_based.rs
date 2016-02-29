extern crate earley;
extern crate env_logger;

use earley::*;

#[derive(PartialEq, Eq)]
pub enum MyToken {
    Plus,
    Times,
    LParen,
    RParen,
    One,
    Two,
    Three,
}

use MyToken::*;

impl Lexeme for MyToken {
    fn matches(&self, other: Self) -> bool {
        self == other;
    }
}

// enum Algebra {
//     Add(Box<Algebra>, Box<Algebra>),
//     Multiply(Box<Algebra>, Box<Algebra>),
//     Number(u64)
// }

// struct MyGrammar;

// trait Grammar<T = &'static str> {
//     type Output;
// }

// impl Grammar<&'static str> for MyGrammar {
//     type Output = Algebra;

//     fn match(terminal,
// }

fn main() {
    env_logger::init().unwrap();

    let rules = vec![
        Production::new("Sum", &[NonTerminal("Sum"), Terminal(Plus), NonTerminal("Product")]),
        Production::new("Sum", &[NonTerminal("Product")]),
        Production::new("Product", &[NonTerminal("Product"), Terminal(Times), NonTerminal("Factor")]),
        Production::new("Product", &[NonTerminal("Factor")]),
        Production::new("Factor", &[Terminal(LParen), NonTerminal("Sum"), Terminal(RParen)]),
        Production::new("Factor", &[NonTerminal("Number")]),
        Production::new("Number", &[Terminal(One)]),
        Production::new("Number", &[Terminal(Two)]),
        Production::new("Number", &[Terminal(Three)]),
    ];

    let grammar = Grammar::new(rules);
    let input = vec![One, Plus, LParen, Two, Times, Three, Plus, Two, RParen];

    let result = parse(grammar.build_table(&input));

    println!("{}", result);
}
