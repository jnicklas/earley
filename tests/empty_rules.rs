#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<String> {
    let productions: Vec<Box<Production<String>>> = vec![
        earley_production!("S" => [{"A"}, {"A"}, ["x"]] (a1, a2, x) -> String; { format!("{}{}{}", a1, a2, x) }),
        earley_production!("A" => [] () -> String; { String::from("moo") }),
    ];

    Grammar::new(productions)
}

#[test]
fn test_empty_rules() {
    let grammar = grammar();

    assert_eq!(grammar.parse("x"), Some(String::from("moomoox")))
}
