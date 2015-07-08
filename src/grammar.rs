use token::{Token, Terminal, NonTerminal};
use item_table::ItemTable;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Grammar {
    pub starting_rule: &'static str,
    pub rules: Vec<Rule>,
}

impl Grammar {
    pub fn build_table(&self, input: &str) -> ItemTable {
        let mut s = ItemTable::new(self, input.len());

        let chars = UnicodeSegmentation::graphemes(input, true).chain(Some("\0").into_iter());

        for (char_index, current_char) in chars.enumerate() {
            debug!("-----> {} matching {}", char_index, current_char);
            let mut item_index = 0;
            while item_index < s.table[char_index].len() {
                let item = s.table[char_index][item_index];
                let next_token = item.next_token();
                debug!("[{}, {}] :: {} || {:?}", char_index, item_index, item, next_token);
                match next_token {
                    Some(&NonTerminal(token)) => s.predict(char_index, token),
                    Some(&Terminal(token)) => s.scan(item, char_index, current_char, token),
                    None => s.complete(item, char_index),
                }
                item_index += 1;
            }
        }

        return s;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rule {
    pub name: &'static str,
    pub tokens: Vec<Token>,
}
