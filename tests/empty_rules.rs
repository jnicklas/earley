#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<String> {
    let productions: Vec<Box<Production<String>>> = vec![
        earley_production!("S" => [NonTerminal("A"), NonTerminal("A"), Terminal("x")] (a1, a2, x) -> String; { format!("{}{}{}", a1.get(), a2.get(), x.value()) }),
        earley_production!("A" => [] () -> String; { String::from("moo") }),
    ];

    Grammar::new(productions)
}

#[test]
fn test_empty_rules() {
    let grammar = grammar();

    assert_eq!(grammar.parse("x"), Some(String::from("moomoox")))
}
