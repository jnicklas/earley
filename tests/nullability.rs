extern crate earley;

use earley::*;

fn grammar() -> Grammar {
    let productions = vec![
        Production::new("A", &[NonTerminal("B"), NonTerminal("C"), NonTerminal("D")]),
        Production::new("B", &[]),
        Production::new("B", &[Terminal("A")]),
        Production::new("C", &[NonTerminal("B")]),
        Production::new("C", &[Terminal("c")]),
        Production::new("D", &[Terminal("d")]),
        Production::new("D", &[NonTerminal("E")]),
        Production::new("E", &[NonTerminal("F")]),
        Production::new("F", &[Terminal("f")]),
    ];

    Grammar::new(productions)
}

#[test]
fn test_basic() {
    let grammar = grammar();

    assert!(!grammar.rules["A"].is_nullable());
    assert!(grammar.rules["B"].is_nullable());
    assert!(grammar.rules["C"].is_nullable());
    assert!(!grammar.rules["D"].is_nullable());
    assert!(!grammar.rules["E"].is_nullable());
    assert!(!grammar.rules["F"].is_nullable());
}