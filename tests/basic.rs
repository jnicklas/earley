#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<u32> {
    let productions: Vec<Box<Production<u32>>> = vec![
        earley_production!("Sum" => {"Sum"}, ["+"], {"Product"};         (result: u32) { result[0].get() + result[2].get() }),
        earley_production!("Sum" => {"Product"};                         (result: u32) { result[0].get() }),
        earley_production!("Product" => {"Product"}, ["*"], {"Factor"};  (result: u32) { result[0].get() * result[2].get() }),
        earley_production!("Product" => {"Factor"};                      (result: u32) { result[0].get() }),
        earley_production!("Factor" => ["("], {"Sum"}, [")"];            (result: u32) { result[1].get() }),
        earley_production!("Factor" => {"Number"};                       (result: u32) { result[0].get() }),
        earley_production!("Number" => ["1"];                            (result: u32) { 1 }),
        earley_production!("Number" => ["2"];                            (result: u32) { 2 }),
        earley_production!("Number" => ["3"];                            (result: u32) { 3 }),
    ];

    Grammar::new(productions)
}

#[test]
fn test_basic() {
    let grammar = grammar();

    assert_eq!(grammar.parse("1+2"), Some(3));
    assert_eq!(grammar.parse("1+(3+2*3)"), Some(10));
}

#[test]
fn test_owned_string() {
    let grammar = grammar();

    let input = format!("{}*{}", 3, 2);

    assert_eq!(grammar.parse(&input), Some(6))
}
