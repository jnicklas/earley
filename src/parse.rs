use item::{Item, Operation};
use item_table::ItemTable;
use token::{Terminal, NonTerminal};
use std::fmt;

#[derive(Debug)]
pub struct Node<'a> {
    item: Item<'a>,
    children: Vec<Node<'a>>
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(self.item.fmt(f));
        for child in &self.children {
            try!("\n".fmt(f));
            for line in format!("{}", child).lines().map(|l| format!("|   {}", l)) {
                try!(line.fmt(f));
                try!("\n".fmt(f));
            }
        }
        Ok(())
    }
}

fn find_edges<'a>(s: &'a ItemTable, mut set: usize, item: Item<'a>) -> Node<'a> {
    let mut node = Node { item: item, children: vec![] };

    node.children = item.get_tokens().iter().rev().map(|token| {
        match token {
            &Terminal(value) => {
                let next_item = s.get_items_in_set(set).iter().cloned().filter(|i| {
                    i.has_same_production(&item) && i.get_operation() == Operation::Scan(value)
                }).nth(0).unwrap();

                set -= 1;

                Node { item: next_item, children: Vec::new() }
            },
            &NonTerminal(name) => {
                let next_item = s.get_items_in_set(set).iter().cloned().filter(|i| {
                    i.is_complete() && i.get_name() == name
                }).nth(0).unwrap();

                let node = find_edges(s, set, next_item);
                set = next_item.get_start();

                node
            }
        }
    }).collect::<Vec<_>>();

    node.children.reverse();

    node
}

pub fn parse<'a>(s: &'a ItemTable) -> Option<Node<'a>> {
    match s.matching_items().into_iter().nth(0) {
        Some(item) => Some(find_edges(s, s.get_input().len(), item)),
        None => None
    }
}
