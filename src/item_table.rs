use std::iter::repeat;
use item::Item;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;
use grammar::{Grammar, RuleName, Terminal, NonTerminal};

pub struct ItemTable<'a, O, N> where O: 'a, N: RuleName {
    input: &'a str,
    table: Vec<Vec<Item<'a, O, N>>>,
    grammar: &'a Grammar<O, N>,
}

impl<'a, O, N> ItemTable<'a, O, N> where O: 'a, N: RuleName {
    pub fn build(grammar: &'a Grammar<O, N>, input: &'a str) -> ItemTable<'a, O, N> {
        let table = repeat(0u8).map(|_| Vec::with_capacity(100)).take(input.len() + 1).collect();
        let mut s = ItemTable { input: input, grammar: grammar, table: table };

        for production in grammar.productions_for_starting_rule() {
            s.push(0, Item::predict(&**production, 0))
        }

        let chars = UnicodeSegmentation::graphemes(input, true).chain(Some("\0").into_iter());

        for (char_index, current_char) in chars.enumerate() {
            let mut item_index = 0;
            while item_index < s.table[char_index].len() {
                let item = s.table[char_index][item_index].clone();
                let next_token = item.next_token();
                match next_token {
                    Some(&NonTerminal(token)) => {
                        s.predict(char_index, token);
                        if grammar.get_rule(token).unwrap().is_nullable() {
                            s.complete_nullable(item.clone(), char_index);
                        }
                    },
                    Some(&Terminal(token)) => s.scan(item.clone(), char_index, current_char, token),
                    None => s.complete(item.clone(), char_index),
                }
                item_index += 1;
            }
        }

        return s;
    }

    pub fn predict(&mut self, char_index: usize, token: N) {
        for production in self.grammar.productions_for(token) {
            self.push(char_index, Item::predict(&**production, char_index));
        }
    }

    pub fn scan(&mut self, item: Item<'a, O, N>, char_index: usize, current_char: &'a str, token: &str) {
        if token == current_char {
            self.push(char_index + 1, item.scan(current_char));
        }
    }

    pub fn complete(&mut self, item: Item<'a, O, N>, char_index: usize) {
        // FIXME: Attack of the clones!
        for old_item in self.table[item.get_start()].clone().iter().cloned() {
            if let Some(&NonTerminal(token)) = old_item.next_token() {
                if token == item.get_name() {
                    self.push(char_index, old_item.complete());
                }
            }
        }
    }

    pub fn complete_nullable(&mut self, item: Item<'a, O, N>, char_index: usize) {
        self.push(char_index, item.complete());
    }

    pub fn matching_items(&self) -> Vec<Item<'a, O, N>> {
        if let Some(items) = self.table.last() {
            items.iter().filter(|item| {
                item.get_name() == self.grammar.get_starting_rule_name() && item.is_complete() && item.get_start() == 0
            }).map(Clone::clone).collect()
        } else {
            Vec::new()
        }
    }

    fn push(&mut self, index: usize, item: Item<'a, O, N>) {
        if let Some(mut items) = self.table.get_mut(index) {
            if !items.contains(&item) {
                items.push(item);
            }
        }
    }

    pub fn get_items_in_set(&'a self, set: usize) -> &'a [Item<'a, O, N>] {
        &self.table[set]
    }

    pub fn get_input(&self) -> &'a str {
        self.input
    }
}

impl<'a, O, N> fmt::Display for ItemTable<'a, O, N> where O: 'a, N: RuleName + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, row) in self.table.iter().enumerate() {
            try!(format!("{:=^80}\n", format!(" {} ", index)).fmt(f));
            for item in row {
                try!(format!("{:<60} {:?}\n", item, item.get_operation()).fmt(f));
            }
        }
        Ok(())
    }
}
