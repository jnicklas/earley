use grammar::Production;
use std::cell::Cell;

pub struct Rule<T> {
    name: &'static str,
    productions: Vec<Box<Production<T>>>,
    nullable: Cell<bool>,
}

impl<T> Rule<T> {
    pub fn new(name: &'static str, productions: Vec<Box<Production<T>>>) -> Rule<T> {
        Rule { name: name, productions: productions.into_iter().collect(), nullable: Cell::new(false) }
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable.get()
    }

    pub fn get_name(&self) -> &'static str {
        &self.name
    }

    pub fn get_productions(&self) -> &[Box<Production<T>>] {
        &self.productions
    }

    pub fn mark_as_nullable(&self) {
        self.nullable.set(true);
    }
}
