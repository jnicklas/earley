#[macro_use]
extern crate earley;

use earley::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Algebra {
    Sum,
    Product,
    Factor,
    Number
}

fn grammar() -> Grammar<u32, Algebra> {
    use Algebra::*;

    let productions: Vec<Box<Production<u32, Algebra>>> = vec![
        earley_production!(Algebra: Sum => [NonTerminal(Sum), Terminal("+"), NonTerminal(Product)]         (a, _, b) -> u32; { a.get() + b.get() }),
        earley_production!(Algebra: Sum => [NonTerminal(Product)]                                          (a) -> u32;       { a.get() }),
        earley_production!(Algebra: Product => [NonTerminal(Product), Terminal("*"), NonTerminal(Factor)]  (a, _, b) -> u32; { a.get() * b.get() }),
        earley_production!(Algebra: Product => [NonTerminal(Factor)]                                       (a) -> u32;       { a.get() }),
        earley_production!(Algebra: Factor => [Terminal("("), NonTerminal(Sum), Terminal(")")]             (_, a, _) -> u32; { a.get() }),
        earley_production!(Algebra: Factor => [NonTerminal(Number)]                                        (a) -> u32;       { a.get() }),
        earley_production!(Algebra: Number => [Terminal("1")]                                              (result) -> u32;  { 1 }),
        earley_production!(Algebra: Number => [Terminal("2")]                                              (result) -> u32;  { 2 }),
        earley_production!(Algebra: Number => [Terminal("3")]                                              (result) -> u32;  { 3 }),
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
