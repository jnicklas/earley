extern crate earley;

use earley::*;

fn grammar() -> Grammar {
    let productions = vec![
        Production::new("A", &[]),
        Production::new("A", &[NonTerminal("B"), NonTerminal("D"), NonTerminal("E"), NonTerminal("F")]),
        Production::new("B", &[NonTerminal("A")]),
        Production::new("C", &[NonTerminal("A")]),
        Production::new("C", &[Terminal("c")]),
        Production::new("D", &[NonTerminal("B"), Terminal("d")]),
        Production::new("E", &[NonTerminal("B")]),
        Production::new("F", &[Terminal("f")]),
    ];

    Grammar::new(productions)
}

#[test]
fn test_basic() {
    let grammar = grammar();

    assert!(grammar.rules["A"].is_nullable());
    assert!(grammar.rules["B"].is_nullable());
    assert!(grammar.rules["C"].is_nullable());
    assert!(grammar.rules["D"].is_nullable());
    assert!(grammar.rules["E"].is_nullable());
    assert!(grammar.rules["F"].is_nullable());
}
