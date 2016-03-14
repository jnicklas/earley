#[macro_use]
extern crate earley;

use earley::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum AlgebraToken {
    Sum,
    Product,
    Factor,
    Number
}

use AlgebraToken::*;

fn main() {
    let grammar = Grammar::new(vec![
        earley_production!(AlgebraToken: Sum => [NonTerminal(Sum), Terminal("+"), NonTerminal(Product)]         (a, _, b) -> u32; { a.get() + b.get() }),
        earley_production!(AlgebraToken: Sum => [NonTerminal(Product)]                                          (a) -> u32;       { a.get() }),
        earley_production!(AlgebraToken: Product => [NonTerminal(Product), Terminal("*"), NonTerminal(Factor)]  (a, _, b) -> u32; { a.get() * b.get() }),
        earley_production!(AlgebraToken: Product => [NonTerminal(Factor)]                                       (a) -> u32;       { a.get() }),
        earley_production!(AlgebraToken: Factor => [Terminal("("), NonTerminal(Sum), Terminal(")")]             (_, a, _) -> u32; { a.get() }),
        earley_production!(AlgebraToken: Factor => [NonTerminal(Number)]                                        (a) -> u32;       { a.get() }),
        earley_production!(AlgebraToken: Number => [Terminal("1")]                                              (result) -> u32;  { 1 }),
        earley_production!(AlgebraToken: Number => [Terminal("2")]                                              (result) -> u32;  { 2 }),
        earley_production!(AlgebraToken: Number => [Terminal("3")]                                              (result) -> u32;  { 3 }),
    ]);

    let input = "1+(2*3+2)";

    println!("{} = {:?}", input, grammar.parse(input));
}
