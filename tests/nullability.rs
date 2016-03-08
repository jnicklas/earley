#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<u32> {
    let productions: Vec<Box<Production<u32>>> = vec![
        earley_production!("A" => [{"B"}, {"C"}, {"D"}] (_result) -> u32; { 1 }),
        earley_production!("B" => [] (_result) -> u32; { 1 }),
        earley_production!("B" => [["a"]] (_result) -> u32; { 1 }),
        earley_production!("C" => [{"B"}] (_result) -> u32; { 1 }),
        earley_production!("C" => [["c"]] (_result) -> u32; { 1 }),
        earley_production!("D" => [["d"]] (_result) -> u32; { 1 }),
        earley_production!("D" => [{"E"}] (_result) -> u32; { 1 }),
        earley_production!("E" => [{"F"}] (_result) -> u32; { 1 }),
        earley_production!("F" => [["f"]] (_result) -> u32; { 1 }),
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
