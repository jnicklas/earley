use grammar::Production;
use std::cell::Cell;
use grammar::RuleName;

pub struct Rule<N, O> where N: RuleName {
    name: N,
    productions: Vec<Box<Production<N, O>>>,
    nullable: Cell<bool>,
}

impl<N, O> Rule<N, O> where N: RuleName {
    pub fn new(name: N, productions: Vec<Box<Production<N, O>>>) -> Rule<N, O> {
        Rule { name: name, productions: productions.into_iter().collect(), nullable: Cell::new(false) }
    }

    pub fn is_nullable(&self) -> bool {
        self.nullable.get()
    }

    pub fn get_name(&self) -> N {
        self.name
    }

    pub fn get_productions(&self) -> &[Box<Production<N, O>>] {
        &self.productions
    }

    pub fn mark_as_nullable(&self) {
        self.nullable.set(true);
    }
}
