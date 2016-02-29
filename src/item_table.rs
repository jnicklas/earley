use std::iter::repeat;
use item::Item;
use grammar::{Grammar, Lexeme};
use token::*;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

pub struct ItemTable<'a, T, I> where T: Lexeme + PartialEq<&'a I>, I: 'a {
    table: Vec<Vec<Item<'a, T, I>>>,
    grammar: &'a Grammar<T>,
}

impl<'a, T, I> ItemTable<'a, T, I> where T: Lexeme + PartialEq<&'a I>, I: 'a {
    pub fn build<A, B>(grammar: &'a Grammar<T>, input: A) -> ItemTable<'a, T, I> where A: IntoIterator<Item=&'a I, IntoIter=B>, B: ExactSizeIterator {
        let iterator = input.into_iter();
        let table = repeat(0u8).map(|_| Vec::with_capacity(100)).take(iterator.len() + 1).collect();
        let mut s = ItemTable { grammar: grammar, table: table };

        for production in grammar.productions_for_starting_rule() {
            s.push(0, Item::predict(production, 0))
        }

        for (input_index, ref current_input) in iterator.enumerate() {
            let mut item_index = 0;
            while item_index < s.table[input_index].len() {
                let item = s.table[input_index][item_index];
                let next_token = item.next_token();
                match next_token {
                    Some(&NonTerminal(token)) => s.predict(input_index, token),
                    Some(&Terminal(token)) => s.scan(item, input_index, current_input, token),
                    None => s.complete(item, input_index),
                }
                if let Some(&NonTerminal(token)) = next_token {
                    if grammar.get_rule(token).unwrap().is_nullable() {
                        s.complete_nullable(item, input_index);
                    }
                }
                item_index += 1;
            }
        }

        return s;
    }

    pub fn predict(&mut self, input_index: usize, token: &str) {
        for rule in self.grammar.productions_for(token) {
            self.push(input_index, Item::predict(rule, input_index));
        }
    }

    pub fn scan(&mut self, item: Item<'a, T, I>, input_index: usize, current_input: &'a I, token: T) {
        if token == current_input {
            self.push(input_index + 1, item.scan(current_input));
        }
    }

    pub fn complete(&mut self, item: Item<'a, T, I>, input_index: usize) {
        // FIXME: the `clone` here is probs super inefficient
        for old_item in self.table[item.get_start()].clone() {
            if let Some(&NonTerminal(token)) = old_item.next_token() {
                if token == item.get_name() {
                    self.push(input_index, old_item.complete());
                }
            }
        }
    }

    pub fn complete_nullable(&mut self, item: Item<'a, T, I>, input_index: usize) {
        self.push(input_index, item.complete());
    }

    pub fn matching_items(&self) -> Vec<Item<'a, T, I>> {
        if let Some(items) = self.table.last() {
            items.iter().filter(|item| {
                item.get_name() == self.grammar.get_starting_rule() && item.is_complete() && item.get_start() == 0
            }).map(Clone::clone).collect()
        } else {
            Vec::new()
        }
    }

    fn push(&mut self, index: usize, item: Item<'a, T, I>) {
        if let Some(mut items) = self.table.get_mut(index) {
            if !items.contains(&item) {
                items.push(item);
            }
        }
    }

    pub fn get_items_in_set(&self, set: usize) -> &[Item<T, I>] {
        &self.table[set]
    }

    pub fn len(&self) -> usize {
        self.table.len() - 1
    }
}

impl<'a, T, I> fmt::Display for ItemTable<'a, T, I> where T: fmt::Display + Lexeme + PartialEq<&'a I>, I: 'a  {
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
