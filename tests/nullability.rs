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

    assert!(!grammar.get_rule("A").unwrap().is_nullable());
    assert!(grammar.get_rule("B").unwrap().is_nullable());
    assert!(grammar.get_rule("C").unwrap().is_nullable());
    assert!(!grammar.get_rule("D").unwrap().is_nullable());
    assert!(!grammar.get_rule("E").unwrap().is_nullable());
    assert!(!grammar.get_rule("F").unwrap().is_nullable());
}
