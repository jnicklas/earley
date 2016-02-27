extern crate earley;

use earley::*;

fn grammar() -> Grammar {
    let productions = vec![
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

    Grammar::new(productions)
}

#[test]
fn test_basic() {
    let grammar = grammar();

    let input = "1*2";
    let table = grammar.build_table(input);
    let result = table.matching_items();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].get_name(), "Sum");
    assert_eq!(result[0].get_tokens(), &[NonTerminal("Product")]);
    assert_eq!(result[0].get_start(), 0);
    assert_eq!(result[0].get_next(), 1);
}

#[test]
fn test_owned_string() {
    let grammar = grammar();

    let input = format!("{}*{}", 1, 2);
    let table = grammar.build_table(&input);
    let result = table.matching_items();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].get_name(), "Sum");
    assert_eq!(result[0].get_tokens(), &[NonTerminal("Product")]);
    assert_eq!(result[0].get_start(), 0);
    assert_eq!(result[0].get_next(), 1);
}

#[test]
fn test_empty_rules() {
    // A simple grammar proposed by Grune & Jacobs. This produces the string `x` but it is not but
    // needs special consideration to be parsed by an Earley parser.
    let productions = vec![
        Production::new("S", &[NonTerminal("A"), NonTerminal("A"), Terminal("x")]),
        Production::new("A", &[]),
    ];

    let grammar = Grammar::new(productions);

    let input = "x";
    if let Some(_node) = parse(&grammar.build_table(input)) {
    } else {
        panic!("unable to parse empty rule");
    }
}
