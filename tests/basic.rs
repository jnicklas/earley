#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<&'static str, u32> {
    Grammar::new(vec![
        earley_production!(&'static str: "Sum" => [NonTerminal("Sum"), Terminal("+"), NonTerminal("Product")]         (a, _, b) -> u32; { a.get() + b.get() }),
        earley_production!(&'static str: "Sum" => [NonTerminal("Product")]                                            (a) -> u32;       { a.get() }),
        earley_production!(&'static str: "Product" => [NonTerminal("Product"), Terminal("*"), NonTerminal("Factor")]  (a, _, b) -> u32; { a.get() * b.get() }),
        earley_production!(&'static str: "Product" => [NonTerminal("Factor")]                                         (a) -> u32;       { a.get() }),
        earley_production!(&'static str: "Factor" => [Terminal("("), NonTerminal("Sum"), Terminal(")")]               (_, a, _) -> u32; { a.get() }),
        earley_production!(&'static str: "Factor" => [NonTerminal("Number")]                                          (a) -> u32;       { a.get() }),
        earley_production!(&'static str: "Number" => [Terminal("1")]                                                  (result) -> u32;  { 1 }),
        earley_production!(&'static str: "Number" => [Terminal("2")]                                                  (result) -> u32;  { 2 }),
        earley_production!(&'static str: "Number" => [Terminal("3")]                                                  (result) -> u32;  { 3 }),
    ])
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
