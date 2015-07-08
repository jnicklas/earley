extern crate earley;
extern crate env_logger;

use earley::*;

fn main() {
    let rules = vec![
        Rule { name: "A", tokens: vec![Terminal("a"), NonTerminal("B")] },
        Rule { name: "B", tokens: Vec::new() },
        Rule { name: "B", tokens: vec![NonTerminal("C")] },
        Rule { name: "C", tokens: vec![Terminal("-")] },
    ];

    let grammar = Grammar { starting_rule: "A", rules: rules };

    let result1 = grammar.build_table("a").matching_items();

    println!("{:?}", result1);
}
