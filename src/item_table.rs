use std::iter::repeat;
use item::Item;
use grammar::Grammar;
use token::NonTerminal;
use std::fmt;

pub struct ItemTable<'a> {
    pub table: Vec<Vec<Item<'a>>>,
    pub grammar: &'a Grammar,
}

impl<'a> ItemTable<'a> {
    pub fn new(grammar: &Grammar, length: usize) -> ItemTable {
        let table = repeat(0u8).map(|_| Vec::with_capacity(100)).take(length + 1).collect();
        let mut s = ItemTable { grammar: grammar, table: table };

        for production in grammar.productions_for_starting_rule() {
            s.push(0, Item::predict(production, 0))
        }

        return s;
    }

    pub fn predict(&mut self, char_index: usize, token: &str) {
        for rule in self.grammar.productions_for(token) {
            self.push(char_index, Item::predict(rule, char_index));
        }
    }

    pub fn scan(&mut self, item: Item<'a>, char_index: usize, current_char: &'a str, token: &str) {
        if token == current_char {
            self.push(char_index + 1, item.scan(current_char));
        }
    }

    pub fn complete(&mut self, item: Item<'a>, char_index: usize) {
        for old_item in self.table[item.start].clone() {
            if let Some(&NonTerminal(token)) = old_item.next_token() {
                if token == item.production.name {
                    self.push(char_index, old_item.complete());
                }
            }
        }
    }

    pub fn complete_nullable(&mut self, item: Item<'a>, char_index: usize) {
        self.push(char_index, item.complete());
    }

    pub fn matching_items(&self) -> Vec<Item<'a>> {
        if let Some(items) = self.table.last() {
            items.iter().filter(|item| {
                item.production.name == self.grammar.starting_rule && item.is_complete() && item.start == 0
            }).map(Clone::clone).collect()
        } else {
            Vec::new()
        }
    }

    fn push(&mut self, index: usize, item: Item<'a>) {
        if let Some(mut items) = self.table.get_mut(index) {
            if !items.contains(&item) {
                debug!("{}", item);
                items.push(item);
            }
        }
    }
}

impl<'a> fmt::Display for ItemTable<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, row) in self.table.iter().enumerate() {
            try!(format!("===== {} ====================================\n", index).fmt(f));
            for item in row {
                let output = format!("{}", item);
                try!(output.fmt(f));
                for _ in 0..(30 - output.len()) {
                    try!(" ".fmt(f));
                }
                try!(format!("{:?}\n", item.operation).fmt(f));
            }
        }
        Ok(())
    }
}
