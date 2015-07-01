extern crate unicode_segmentation;
#[macro_use] extern crate log;

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

impl Item {
    fn new(rule: usize, start: usize) -> Item {
        Item { rule: rule, next: 0, start: start }
    }

    fn advance(&self) -> Item {
        Item { rule: self.rule, next: self.next + 1, start: self.start }
    }
}

pub struct ItemTable<'a> {
    pub table: Vec<Vec<Item>>,
    pub grammar: &'a Grammar,
}


impl<'a> ItemTable<'a> {
    fn new(grammar: &Grammar, length: usize) -> ItemTable {
        ItemTable { grammar: grammar, table: vec![vec![]; length + 1] }
    }

    fn push(&mut self, operation: &str, index: usize, item: Item) {
        if let Some(mut items) = self.table.get_mut(index) {
            if !items.contains(&item) {
                debug!("|- {} :: {}", operation, render_item(self.grammar, &item));
                items.push(item);
            }
        }
    }

    fn predict(&mut self, char_index: usize, token: &str) {
        for (rule_index, rule) in self.grammar.rules.iter().enumerate() {
            if rule.name == token {
                self.push("predicting", char_index, Item::new(rule_index, char_index));
            }
        }
    }

    fn scan(&mut self, item: Item, char_index: usize, current_char: &str, token: &str) {
        if token == current_char {
            self.push("scanning", char_index + 1, item.advance());
        }
    }

    fn complete(&mut self, item: Item, char_index: usize) {
        for old_item in self.table[item.start].clone() {
            if let Some(&NonTerminal(token)) = self.grammar.rules[old_item.rule].tokens.get(old_item.next) {
                if token == self.grammar.rules[item.rule].name {
                    self.push("completing", char_index, old_item.advance());
                }
            }
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

pub fn build_items<'a>(grammar: &'a Grammar, input: &str) -> ItemTable<'a> {
    let mut s = ItemTable::new(grammar, input.len());

    for (rule_index, _) in grammar.rules.iter().filter(|r| r.name == grammar.starting_rule).enumerate() {
        s.push("initializing", 0, Item::new(rule_index, 0))
    }

    let chars = UnicodeSegmentation::graphemes(input, true).chain(Some("\0").into_iter());

    for (char_index, current_char) in chars.enumerate() {
        debug!("-----> {} matching {}", char_index, current_char);
        let mut item_index = 0;
        while item_index < s.table[char_index].len() {
            let item = s.table[char_index][item_index];
            let next_item = grammar.rules[item.rule].tokens.get(item.next);
            debug!("[{}, {}] :: {} || {:?}", char_index, item_index, render_item(&grammar, &item), next_item);
            match next_item {
                Some(&NonTerminal(token)) => s.predict(char_index, token),
                Some(&Terminal(token)) => s.scan(item, char_index, current_char, token),
                None => s.complete(item, char_index),
            }
            item_index += 1;
        }
    }

    return s;
}

pub fn matching_items(s: &ItemTable) -> Vec<Item> {
    if let Some(items) = s.table.last() {
       items.iter().filter(|item| {
           let rule = &s.grammar.rules[item.rule];
           rule.name == s.grammar.starting_rule && item.next >= rule.tokens.len() && item.start == 0
       }).map(Clone::clone).collect()
    } else {
        Vec::new()
    }
}
