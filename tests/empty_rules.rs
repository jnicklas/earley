#[macro_use]
extern crate earley;

use earley::*;

fn grammar() -> Grammar<&'static str, String> {
    Grammar::new(vec![
        earley_production!(&'static str: "S" => [NonTerminal("A"), NonTerminal("A"), Terminal('x')] (a1, a2, x) -> String; { format!("{}{}{}", a1.get(), a2.get(), x.value()) }),
        earley_production!(&'static str: "A" => [] () -> String; { String::from("moo") }),
    ])

}

#[test]
fn test_empty_rules() {
    let grammar = grammar();

    assert_eq!(grammar.parse("x"), Some(String::from("moomoox")))
}
