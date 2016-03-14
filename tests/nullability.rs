#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<&'static str, u32> {
    Grammar::new(vec![
        earley_production!(&'static str: "A" => [NonTerminal("B"), NonTerminal("C"), NonTerminal("D")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "B" => [] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "B" => [Terminal("a")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "C" => [NonTerminal("B")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "C" => [Terminal("c")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "D" => [Terminal("d")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "D" => [NonTerminal("E")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "E" => [NonTerminal("F")] (_result) -> u32; { 1 }),
        earley_production!(&'static str: "F" => [Terminal("f")] (_result) -> u32; { 1 }),
    ])
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
