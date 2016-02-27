
extern crate earley;
extern crate env_logger;

use earley::*;

fn main() {
    env_logger::init().unwrap();

    let productions = vec![
        Production::new("S", &[NonTerminal("A"), NonTerminal("A"), Terminal("x")]),
        Production::new("A", &[]),
    ];

    let grammar = Grammar::new(productions);

    let table = grammar.build_table("x");

    let items = table.matching_items();

    println!("{}", table);
    println!("\n-- Start item\n");
    for item in &items {
        println!("{}", item);
    }

    println!("\n-- Parse tree\n");

    for node in parse(&table) {
        println!("{}", node);
    }
}
