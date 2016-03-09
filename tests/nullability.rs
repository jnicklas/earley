#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<u32> {
    let productions: Vec<Box<Production<u32>>> = vec![
        earley_production!("A" => [{"B"}, {"C"}, {"D"}] (_, _, _) -> u32; { 1 }),
        earley_production!("B" => []                    () -> u32;        { 1 }),
        earley_production!("B" => [["a"]]               (_) -> u32;       { 1 }),
        earley_production!("C" => [{"B"}]               (_) -> u32;       { 1 }),
        earley_production!("C" => [["c"]]               (_) -> u32;       { 1 }),
        earley_production!("D" => [["d"]]               (_) -> u32;       { 1 }),
        earley_production!("D" => [{"E"}]               (_) -> u32;       { 1 }),
        earley_production!("E" => [{"F"}]               (_) -> u32;       { 1 }),
        earley_production!("F" => [["f"]]               (_) -> u32;       { 1 }),
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
