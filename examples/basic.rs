#[macro_use]
extern crate earley;
extern crate env_logger;

use earley::*;

fn main() {
    env_logger::init().unwrap();

    let grammar = Grammar::new(vec![
        earley_production!("Sum" => [NonTerminal("Sum"), Terminal("+"), NonTerminal("Product")]         (a, _, b) -> u32; { a.get() + b.get() }),
        earley_production!("Sum" => [NonTerminal("Product")]                                            (a) -> u32;       { a.get() }),
        earley_production!("Product" => [NonTerminal("Product"), Terminal("*"), NonTerminal("Factor")]  (a, _, b) -> u32; { a.get() * b.get() }),
        earley_production!("Product" => [NonTerminal("Factor")]                                         (a) -> u32;       { a.get() }),
        earley_production!("Factor" => [Terminal("("), NonTerminal("Sum"), Terminal(")")]               (_, a, _) -> u32; { a.get() }),
        earley_production!("Factor" => [NonTerminal("Number")]                                          (a) -> u32;       { a.get() }),
        earley_production!("Number" => [Terminal("1")]                                                  (result) -> u32;  { 1 }),
        earley_production!("Number" => [Terminal("2")]                                                  (result) -> u32;  { 2 }),
        earley_production!("Number" => [Terminal("3")]                                                  (result) -> u32;  { 3 }),
    ]);

    let input = "1+(2*3+2)";

    println!("{} = {:?}", input, grammar.parse(input));
}
