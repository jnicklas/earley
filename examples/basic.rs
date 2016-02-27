extern crate earley;
extern crate env_logger;

use earley::{Rule, Grammar, Terminal, NonTerminal, parse};

fn main() {
    env_logger::init().unwrap();

    let rules = vec![
        Rule::new("Sum", &[NonTerminal("Sum"), Terminal("+"), NonTerminal("Product")]),
        Rule::new("Sum", &[NonTerminal("Product")]),
        Rule::new("Product", &[NonTerminal("Product"), Terminal("*"), NonTerminal("Factor")]),
        Rule::new("Product", &[NonTerminal("Factor")]),
        Rule::new("Factor", &[Terminal("("), NonTerminal("Sum"), Terminal(")")]),
        Rule::new("Factor", &[NonTerminal("Number")]),
        Rule::new("Number", &[Terminal("1")]),
        Rule::new("Number", &[Terminal("2")]),
        Rule::new("Number", &[Terminal("3")]),
    ];

    let grammar = Grammar::new("Sum", &rules);

    let input = "1+(2*3+2)";
    let items = grammar.build_table(input);

    println!("--------------------");
    for (index, items) in items.table.iter().enumerate() {
        println!("===== {} =====", index);
        for item in items {
            if item.is_complete() {
                println!("{}", item);
            }
        }

    }
    println!("--------------------");
    let node = parse(&items);
    println!("--------------------");
    println!("{}", node.unwrap());
}
