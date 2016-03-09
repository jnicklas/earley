#[macro_use]
extern crate earley;
extern crate env_logger;

use earley::*;

fn main() {
    env_logger::init().unwrap();

    let grammar = Grammar::new(vec![
        earley_production!("Sum" => [{"Sum"}, ["+"], {"Product"}]         (a, _, b) -> u32; { a + b }),
        earley_production!("Sum" => [{"Product"}]                         (a) -> u32;       { a }),
        earley_production!("Product" => [{"Product"}, ["*"], {"Factor"}]  (a, _, b) -> u32; { a * b }),
        earley_production!("Product" => [{"Factor"}]                      (a) -> u32;       { a }),
        earley_production!("Factor" => [["("], {"Sum"}, [")"]]            (_, a, _) -> u32; { a }),
        earley_production!("Factor" => [{"Number"}]                       (a) -> u32;       { a }),
        earley_production!("Number" => [["1"]]                            (result) -> u32;  { 1 }),
        earley_production!("Number" => [["2"]]                            (result) -> u32;  { 2 }),
        earley_production!("Number" => [["3"]]                            (result) -> u32;  { 3 }),
    ]);

    let input = "1+(2*3+2)";

    println!("{} = {:?}", input, grammar.parse(input));
}
