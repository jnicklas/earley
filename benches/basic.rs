#![feature(test)]

extern crate test;
extern crate earley;

use earley::*;

fn grammar() -> Grammar {
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

    Grammar { starting_rule: "Sum", rules: rules }
}

#[bench]
fn bench_basic(b: &mut ::test::Bencher) {
    let grammar = grammar();

    let input = "1*2";

    b.iter(|| {
        let items = build_items(&grammar, input);
        matching_items(&items);
    })
}

#[bench]
fn bench_long_input(b: &mut ::test::Bencher) {
    let grammar = grammar();

    let input = "1*2+((1+2)*3)+2+2*4";

    b.iter(|| {
        let items = build_items(&grammar, input);
        matching_items(&items);
    })
}
