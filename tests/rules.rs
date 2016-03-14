#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<u32, &'static str> {
    Grammar::new(vec![
        earley_production!(&'static str: "A" => [Terminal("a")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "A" => [NonTerminal("B")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "C" => [Terminal("c")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "B" => [NonTerminal("C")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "C" => [Terminal("d")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "C" => [Terminal("e")] (_result) -> u32; { 1 }),
    ])
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
