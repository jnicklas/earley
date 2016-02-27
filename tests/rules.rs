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

    assert_eq!(grammar.rules.len(), 3);
    assert_eq!(grammar.rules["A"].productions.len(), 2);
    assert_eq!(grammar.rules["B"].productions.len(), 1);
    assert_eq!(grammar.rules["C"].productions.len(), 3);

    assert_eq!(grammar.rules["A"].productions[0].tokens, vec![Terminal("a")]);
    assert_eq!(grammar.rules["A"].productions[1].tokens, vec![NonTerminal("B")]);
    assert_eq!(grammar.rules["B"].productions[0].tokens, vec![NonTerminal("C")]);
    assert_eq!(grammar.rules["C"].productions[0].tokens, vec![Terminal("c")]);
    assert_eq!(grammar.rules["C"].productions[1].tokens, vec![Terminal("d")]);
    assert_eq!(grammar.rules["C"].productions[2].tokens, vec![Terminal("e")]);
}
