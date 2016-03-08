#[macro_use]
extern crate earley;
extern crate env_logger;

use earley::*;

fn main() {
    env_logger::init().unwrap();

    let grammar = Grammar::new(vec![
        earley_production!("Sum" => [{"Sum"}, ["+"], {"Product"}]         (result: u32) { result[0].get() + result[2].get() }),
        earley_production!("Sum" => [{"Product"}]                         (result: u32) { result[0].get() }),
        earley_production!("Product" => [{"Product"}, ["*"], {"Factor"}]  (result: u32) { result[0].get() * result[2].get() }),
        earley_production!("Product" => [{"Factor"}]                      (result: u32) { result[0].get() }),
        earley_production!("Factor" => [["("], {"Sum"}, [")"]]            (result: u32) { result[1].get() }),
        earley_production!("Factor" => [{"Number"}]                       (result: u32) { result[0].get() }),
        earley_production!("Number" => [["1"]]                            (result: u32) { 1 }),
        earley_production!("Number" => [["2"]]                            (result: u32) { 2 }),
        earley_production!("Number" => [["3"]]                            (result: u32) { 3 }),
    ]);

    let input = "1+(2*3+2)";

    println!("{} = {:?}", input, grammar.parse(input));
}
