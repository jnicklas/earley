extern crate earley;

use earley::*;

fn grammar() -> Grammar {
    let rules = vec![
        Rule { name: "Sum", tokens: vec![NonTerminal("Sum"), Terminal("+"), NonTerminal("Product")] },
        Rule { name: "Sum", tokens: vec![NonTerminal("Product")] },
        Rule { name: "Product", tokens: vec![NonTerminal("Product"), Terminal("*"), NonTerminal("Factor")] },
        Rule { name: "Product", tokens: vec![NonTerminal("Factor")] },
        Rule { name: "Factor", tokens: vec![Terminal("("), NonTerminal("Sum"), Terminal(")")] },
        Rule { name: "Factor", tokens: vec![NonTerminal("Number")] },
        Rule { name: "Number", tokens: vec![Terminal("1")] },
        Rule { name: "Number", tokens: vec![Terminal("2")] },
        Rule { name: "Number", tokens: vec![Terminal("3")] },
    ];

    Grammar { starting_rule: "Sum", rules: rules }
}

#[test]
fn test_basic() {
    let grammar = grammar();

    let input = "1*2";
    let table = grammar.build_table(input);
    let result = table.matching_items();

    assert_eq!(result.len(), 1);
    assert_eq!(*result[0].rule, grammar.rules[1]);
    assert_eq!(result[0].start, 0);
    assert_eq!(result[0].next, 1);
}

#[test]
fn test_owned_string() {
    let grammar = grammar();

    let input = format!("{}*{}", 1, 2);
    let table = grammar.build_table(&input);
    let result = table.matching_items();

    assert_eq!(result.len(), 1);
    assert_eq!(*result[0].rule, grammar.rules[1]);
    assert_eq!(result[0].start, 0);
    assert_eq!(result[0].next, 1);
}

// #[test]
// fn test_empty_rules() {
//     let rules = vec![
//         Rule { name: "A", tokens: vec![Terminal("a"), NonTerminal("B")] },
//         Rule { name: "B", tokens: Vec::new() },
//         Rule { name: "B", tokens: vec![NonTerminal("C")] },
//         Rule { name: "C", tokens: vec![Terminal("-")] },
//     ];

//     let grammar = Grammar { starting_rule: "A", rules: rules };

//     let input = "a-";
//     let items = build_items(&grammar, input);
//     let result = matching_items(&items);

//     println!("{:?}", result);

//     assert_eq!(result.len(), 1);
//     assert_eq!(result[0].rule, 1);
//     assert_eq!(result[0].start, 0);
//     assert_eq!(result[0].next, 1);
// }
