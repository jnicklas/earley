#![feature(test)]

extern crate test;
extern crate earley;
extern crate env_logger;

use earley::{Rule, Grammar, Terminal, NonTerminal};

fn main() {
    env_logger::init().unwrap();

    let rules = vec![
        Rule { name: "Sum", tokens: vec![NonTerminal("Sum"), Terminal("+"), NonTerminal("Product")] },
        Rule { name: "Sum", tokens: vec![NonTerminal("Product")] },
        Rule { name: "Product", tokens: vec![NonTerminal("Product"), Terminal("*"), NonTerminal("Factor")] },
        Rule { name: "Product", tokens: vec![NonTerminal("Factor")] },
        Rule { name: "Factor", tokens: vec![Terminal("("), NonTerminal("Sum"), Terminal(")")] },
        Rule { name: "Factor", tokens: vec![NonTerminal("Number")] },
        Rule { name: "Number", tokens: vec![Terminal("1")] },
        Rule { name: "Number", tokens: vec![Terminal("2")] },
        Rule { name: "Number", tokens: vec![Terminal("3")] },
    ];

    let grammar = Grammar {
        starting_rule: "Sum",
        rules: rules,
    };

    let input = "1*2+((1+2)*3)+2+2*4";

    for _i in 0..3000 {
        let table = grammar.build_table(input);
        let result = table.matching_items();
        test::black_box(result);
    }

    println!("done 2!");
}
