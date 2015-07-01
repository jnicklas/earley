extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    Terminal(&'static str),
    NonTerminal(&'static str),
}

impl Token {
    pub fn name(&self) -> String {
        match *self {
            Terminal(str) => format!("'{}'", str),
            NonTerminal(str) => str.into()
        }
    }
}

pub use Token::*;

#[derive(Debug)]
pub struct Rule {
    pub name: &'static str,
    pub tokens: Vec<Token>,
}

#[derive(Debug)]
pub struct Grammar {
    pub starting_rule: &'static str,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item {
    rule: usize,
    start: usize,
    next: usize,
}

pub type ItemTable = Vec<Vec<Item>>;

fn push(operation: &str, grammar: &Grammar, s: &mut ItemTable, index: usize, item: Item) {
    if let Some(mut items) = s.get_mut(index) {
        if !items.contains(&item) {
            println!("|- {} :: {}", operation, render_item(grammar, &item));
            items.push(item);
        }
    }
}

fn render_item(grammar: &Grammar, item: &Item) -> String {
    let rule = &grammar.rules[item.rule];
    let mut tokens: Vec<String> = rule.tokens.iter().map(Token::name).collect();
    if item.next < tokens.len() {
        tokens.insert(item.next, "*".to_string());
    } else {
        tokens.push("*".to_string());
    }
    format!("{} -> {}", rule.name, tokens.connect(" "))
}

pub fn build_items(grammar: &Grammar, input: &str) -> ItemTable {
    let mut s = vec![vec![]; input.len() + 1];

    for (rule_index, rule) in grammar.rules.iter().filter(|r| r.name == grammar.starting_rule).enumerate() {
        s[0].push(Item { rule: rule_index, start: 0, next: 0 })
    }

    for (char_index, current_char) in UnicodeSegmentation::graphemes(&*format!("{} ", input), true).enumerate() {
        println!("-----> {} matching {}", char_index, current_char);
        let mut item_index = 0;
        while item_index < s[char_index].len() {
            let item = s[char_index][item_index];
            println!("[{}, {}] :: {} || {:?}", char_index, item_index, render_item(&grammar, &item), grammar.rules[item.rule].tokens.get(item.next));
            match grammar.rules[item.rule].tokens.get(item.next) {
                // Predict
                Some(&NonTerminal(token)) => {
                    for (rule_index, rule) in grammar.rules.iter().enumerate() {
                        if rule.name == token {
                            push("predicting", grammar, &mut s, char_index, Item { rule: rule_index, next: 0, start: char_index });
                        }
                    }
                },
                // Scan
                Some(&Terminal(token)) => {
                    if token == current_char {
                        push("scanning", grammar, &mut s, char_index + 1, Item { rule: item.rule, next: item.next + 1, start: item.start });
                    }
                },
                // Complete
                None => {
                    for old_item in s[item.start].clone() {
                        match grammar.rules[old_item.rule].tokens.get(old_item.next) {
                            Some(&NonTerminal(token)) => {
                                if token == grammar.rules[item.rule].name {
                                    push("completing", grammar, &mut s, char_index, Item { rule: old_item.rule, next: old_item.next + 1, start: old_item.start });
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
            item_index += 1;
        }
    }

    return s;
}

pub fn matching_items(grammar: &Grammar, items: &ItemTable) -> Vec<Item> {
    if let Some(items) = items.last() {
       items.iter().filter(|item| {
           let rule = &grammar.rules[item.rule];
           rule.name == grammar.starting_rule && item.next >= rule.tokens.len() && item.start == 0
       }).map(Clone::clone).collect()
    } else {
        Vec::new()
    }
}
