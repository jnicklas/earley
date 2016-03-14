use grammar::Production;
use std::cell::Cell;
use grammar::RuleName;

pub struct Rule<T, K> where K: RuleName {
    name: K,
    productions: Vec<Box<Production<T, K>>>,
    nullable: Cell<bool>,
}

impl<T, K> Rule<T, K> where K: RuleName {
    pub fn new(name: K, productions: Vec<Box<Production<T, K>>>) -> Rule<T, K> {
        Rule { name: name, productions: productions.into_iter().collect(), nullable: Cell::new(false) }
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable.get()
    }

    pub fn get_name(&self) -> K {
        self.name
    }

    pub fn get_productions(&self) -> &[Box<Production<T, K>>] {
        &self.productions
    }

    pub fn mark_as_nullable(&self) {
        self.nullable.set(true);
    }
}
