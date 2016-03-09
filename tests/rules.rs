#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<u32> {
    let productions: Vec<Box<Production<u32>>> = vec![
        earley_production!("A" => [Terminal("a")] (_result) -> u32; { 1 }),
        earley_production!("A" => [NonTerminal("B")] (_result) -> u32; { 1 }),
        earley_production!("C" => [Terminal("c")] (_result) -> u32; { 1 }),
        earley_production!("B" => [NonTerminal("C")] (_result) -> u32; { 1 }),
        earley_production!("C" => [Terminal("d")] (_result) -> u32; { 1 }),
        earley_production!("C" => [Terminal("e")] (_result) -> u32; { 1 }),
    ];

    Grammar::new(productions)
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
