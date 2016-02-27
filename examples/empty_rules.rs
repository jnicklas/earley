extern crate earley;
extern crate env_logger;

use earley::*;

fn main() {
    let rules = vec![
        Production::new("A", &[Terminal("a"), NonTerminal("B"), NonTerminal("C")]),
        Production::new("B", &[]),
        Production::new("B", &[Terminal("b")]),
        Production::new("C", &[Terminal("-")]),
    ];

    let grammar = Grammar::new(rules);

    let table = grammar.build_table("a-");

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
