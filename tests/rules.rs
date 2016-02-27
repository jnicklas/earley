extern crate earley;

use earley::*;

fn grammar() -> Grammar {
    let rules = vec![
        Production::new("A", &[Terminal("a")]),
        Production::new("A", &[NonTerminal("B")]),
        Production::new("C", &[Terminal("c")]),
        Production::new("B", &[NonTerminal("C")]),
        Production::new("C", &[Terminal("d")]),
        Production::new("C", &[Terminal("e")]),
    ];

    Grammar::new(rules)
}

#[test]
fn test_basic() {
    let grammar = grammar();

    assert_eq!(grammar.get_rule("A").unwrap().get_productions().len(), 2);
    assert_eq!(grammar.get_rule("B").unwrap().get_productions().len(), 1);
    assert_eq!(grammar.get_rule("C").unwrap().get_productions().len(), 3);

    assert_eq!(grammar.get_rule("A").unwrap().get_productions()[0].get_tokens(), &[Terminal("a")]);
    assert_eq!(grammar.get_rule("A").unwrap().get_productions()[1].get_tokens(), &[NonTerminal("B")]);
    assert_eq!(grammar.get_rule("B").unwrap().get_productions()[0].get_tokens(), &[NonTerminal("C")]);
    assert_eq!(grammar.get_rule("C").unwrap().get_productions()[0].get_tokens(), &[Terminal("c")]);
    assert_eq!(grammar.get_rule("C").unwrap().get_productions()[1].get_tokens(), &[Terminal("d")]);
    assert_eq!(grammar.get_rule("C").unwrap().get_productions()[2].get_tokens(), &[Terminal("e")]);
}
