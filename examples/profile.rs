// #![feature(test)]

// extern crate test;
extern crate earley;
extern crate env_logger;

use earley::{Rule, Grammar, Terminal, NonTerminal};

fn main() {
    env_logger::init().unwrap();

    let rules = vec![
        Rule::new("Sum", &[NonTerminal("Sum"), Terminal("+"), NonTerminal("Product")]),
        Rule::new("Sum", &[NonTerminal("Product")]),
        Rule::new("Product", &[NonTerminal("Product"), Terminal("*"), NonTerminal("Factor")]),
        Rule::new("Product", &[NonTerminal("Factor")]),
        Rule::new("Factor", &[Terminal("("), NonTerminal("Sum"), Terminal(")")]),
        Rule::new("Factor", &[NonTerminal("Number")]),
        Rule::new("Number", &[Terminal("1")]),
        Rule::new("Number", &[Terminal("2")]),
        Rule::new("Number", &[Terminal("3")]),
    ];

    let grammar = Grammar::new("Sum", &rules);

    let input = "1*2+((1+2)*3)+2+2*4";

    for _i in 0..3000 {
        let table = grammar.build_table(input);
        let _result = table.matching_items();
        // test::black_box(result);
    }

    println!("done 2!");
}
