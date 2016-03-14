use item::{Item, Operation};
use item_table::ItemTable;
use grammar::{RuleName, Terminal, NonTerminal};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Value<'a, O> {
    Terminal(&'a str),
    NonTerminal(O),
}

impl<'a, O> Value<'a, O> where O: 'a {
    pub fn get(self) -> O {
        match self {
            Value::Terminal(_) => panic!("cannot call `get()` on terminal nodes"),
            Value::NonTerminal(value) => value,
        }
    }

    pub fn value(self) -> &'a str {
        match self {
            Value::Terminal(value) => value,
            Value::NonTerminal(_) => panic!("cannot call `value()` on non-terminal nodes"),
        }
    }
}

fn find_edges<'a, O, N>(s: &'a ItemTable<'a, O, N>, mut set: usize, item: Item<'a, O, N>) -> O where O: 'a, N: RuleName {
    let tokens = item.get_tokens();
    let mut children = Vec::with_capacity(tokens.len());

    for token in tokens.iter().rev() {
        match token {
            &Terminal(value) => {
                let next_item = s.get_items_in_set(set).iter().cloned().filter(|i| {
                    i.has_same_production(&item) && i.get_operation() == Operation::Scan(value)
                }).nth(0).unwrap();

                set -= 1;

                children.push(Value::Terminal(next_item.get_scanned_value().unwrap()));
            },
            &NonTerminal(name) => {
                let next_item = s.get_items_in_set(set).iter().cloned().filter(|i| {
                    i.is_complete() && i.get_name() == name
                }).nth(0).unwrap();

                let value = find_edges(s, set, next_item.clone());

                set = next_item.get_start();

                children.push(Value::NonTerminal(value));
            }
        }
    }

    children.reverse(); // FIXME: this reverse is a bit ugly

    item.perform(children)
}

pub fn parse<'a, O, N>(s: &'a ItemTable<O, N>) -> Option<O> where O: 'a, N: RuleName {
    match s.matching_items().into_iter().nth(0) {
        Some(item) => {
            Some(find_edges(s, s.get_input().len(), item.clone()))
        },
        None => None
    }
}
