use item::{Item, Operation};
use item_table::ItemTable;
use token::{Terminal, NonTerminal};
use std::fmt;

pub struct Node<'a, T> where T: 'a {
    item: Item<'a, T>,
    children: Vec<Node<'a, T>>
}

impl<'a, T> Node<'a, T> where T: 'a {
    pub fn get(&self) -> T {
        if self.children.len() == 0 {
            panic!("cannot call `get()` on terminal nodes");
        }
        self.item.perform(&self.children)
    }

    pub fn value(&'a self) -> &'a str {
        match self.item.get_operation() {
            Operation::Scan(value) => value,
            _ => panic!("can't get value if it wasn't a scan")
        }
    }
}

impl<'a, T> fmt::Display for Node<'a, T> where T: 'a {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(self.item.fmt(f));
        for child in &self.children {
            try!("\n".fmt(f));
            for line in child.to_string().lines().map(|l| format!("|   {}", l)) {
                try!(line.fmt(f));
                try!("\n".fmt(f));
            }
        }
        Ok(())
    }
}

fn find_edges<'a, T>(s: &'a ItemTable<'a, T>, mut set: usize, item: Item<'a, T>) -> Node<'a, T> where T: 'a {
    let mut node = Node { item: item.clone(), children: vec![] };

    node.children = item.get_tokens().iter().rev().map(|token| {
        match token {
            &Terminal(value) => {
                let next_item = s.get_items_in_set(set).iter().cloned().filter(|i| {
                    i.has_same_production(&item) && i.get_operation() == Operation::Scan(value)
                }).nth(0).unwrap();

                set -= 1;

                Node { item: next_item.clone(), children: Vec::new() }
            },
            &NonTerminal(name) => {
                let next_item = s.get_items_in_set(set).iter().cloned().filter(|i| {
                    i.is_complete() && i.get_name() == name
                }).nth(0).unwrap();

                let node = find_edges(s, set, next_item.clone());
                set = next_item.get_start();

                node
            }
        }
    }).collect::<Vec<_>>();

    node.children.reverse();

    node
}

pub fn parse<'a, T>(s: &'a ItemTable<T>) -> Option<T> where T: 'a {
    match s.matching_items().into_iter().nth(0) {
        Some(item) => Some(find_edges(s, s.get_input().len(), item).get()),
        None => None
    }
}
