extern crate earley;
extern crate env_logger;

use earley::*;

fn main() {
    env_logger::init().unwrap();

    let rules = vec![
        Production::new("Sum", &[NonTerminal("Sum"), Terminal("+"), NonTerminal("Product")]),
        Production::new("Sum", &[NonTerminal("Product")]),
        Production::new("Product", &[NonTerminal("Product"), Terminal("*"), NonTerminal("Factor")]),
        Production::new("Product", &[NonTerminal("Factor")]),
        Production::new("Factor", &[Terminal("("), NonTerminal("Sum"), Terminal(")")]),
        Production::new("Factor", &[NonTerminal("Number")]),
        Production::new("Number", &[Terminal("1")]),
        Production::new("Number", &[Terminal("2")]),
        Production::new("Number", &[Terminal("3")]),
    ];

    let grammar = Grammar::new(rules);

    let input = "1+(2*3+2)";
    let items = grammar.build_table(input);

    println!("--------------------");
    println!("{}", items);
    println!("--------------------");
    let node = parse(&items);
    println!("--------------------");
    println!("{}", node.unwrap());
}
