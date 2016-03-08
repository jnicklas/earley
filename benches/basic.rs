#![feature(test)]

#[macro_use]
extern crate earley;

extern crate test;

use earley::*;

fn grammar() -> Grammar<u32> {
    let productions: Vec<Box<Production<u32>>> = vec![
        earley_production!("Sum" => [{"Sum"}, ["+"], {"Product"}]         (result: u32) { result[0].get() + result[2].get() }),
        earley_production!("Sum" => [{"Product"}]                         (result: u32) { result[0].get() }),
        earley_production!("Product" => [{"Product"}, ["*"], {"Factor"}]  (result: u32) { result[0].get() * result[2].get() }),
        earley_production!("Product" => [{"Factor"}]                      (result: u32) { result[0].get() }),
        earley_production!("Factor" => [["("], {"Sum"}, [")"]]            (result: u32) { result[1].get() }),
        earley_production!("Factor" => [{"Number"}]                       (result: u32) { result[0].get() }),
        earley_production!("Number" => [["1"]]                            (result: u32) { 1 }),
        earley_production!("Number" => [["2"]]                            (result: u32) { 2 }),
        earley_production!("Number" => [["3"]]                            (result: u32) { 3 }),
    ];

    Grammar::new(productions)
}

#[bench]
fn bench_basic(b: &mut ::test::Bencher) {
    let grammar = grammar();

    let input = "1*2";

    b.iter(|| {
        grammar.parse(input)
    })
}

#[bench]
fn bench_long_input(b: &mut ::test::Bencher) {
    let grammar = grammar();

    let input = "1*2+((1+2)*3)+2+2*4";

    b.iter(|| {
        grammar.parse(input)
    })
}
