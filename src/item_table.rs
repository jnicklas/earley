use item::Item;
use grammar::Grammar;
use token::NonTerminal;

pub struct ItemTable<'a> {
    pub table: Vec<Vec<Item>>,
    pub grammar: &'a Grammar,
}

impl<'a> ItemTable<'a> {
    pub fn new(grammar: &Grammar, length: usize) -> ItemTable {
        let mut s = ItemTable { grammar: grammar, table: vec![vec![]; length + 1] };

        for (rule_index, _) in grammar.rules.iter().filter(|r| r.name == grammar.starting_rule).enumerate() {
            s.push("initializing", 0, Item::new(rule_index, 0))
        }

        return s;
    }

    pub fn predict(&mut self, char_index: usize, token: &str) {
        for (rule_index, rule) in self.grammar.rules.iter().enumerate() {
            if rule.name == token {
                self.push("predicting", char_index, Item::new(rule_index, char_index));
            }
        }
    }

    pub fn scan(&mut self, item: Item, char_index: usize, current_char: &str, token: &str) {
        if token == current_char {
            self.push("scanning", char_index + 1, item.advance());
        }
    }

    pub fn complete(&mut self, item: Item, char_index: usize) {
        for old_item in self.table[item.start].clone() {
            if let Some(&NonTerminal(token)) = self.grammar.rules[old_item.rule].tokens.get(old_item.next) {
                if token == self.grammar.rules[item.rule].name {
                    self.push("completing", char_index, old_item.advance());
                }
            }
        }
    }

    fn push(&mut self, operation: &str, index: usize, item: Item) {
        if let Some(mut items) = self.table.get_mut(index) {
            if !items.contains(&item) {
                debug!("|- {} :: {}", operation, item.render(self.grammar));
                items.push(item);
            }
        }
    }
}

