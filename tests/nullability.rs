#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<u32> {
    let productions: Vec<Box<Production<u32>>> = vec![
        earley_production!("A" => [{"B"}, {"C"}, {"D"}] (result: u32) { 1 }),
        earley_production!("B" => [] (result: u32) { 1 }),
        earley_production!("B" => [["a"]] (result: u32) { 1 }),
        earley_production!("C" => [{"B"}] (result: u32) { 1 }),
        earley_production!("C" => [["c"]] (result: u32) { 1 }),
        earley_production!("D" => [["d"]] (result: u32) { 1 }),
        earley_production!("D" => [{"E"}] (result: u32) { 1 }),
        earley_production!("E" => [{"F"}] (result: u32) { 1 }),
        earley_production!("F" => [["f"]] (result: u32) { 1 }),
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
