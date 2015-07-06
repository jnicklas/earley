use grammar::Grammar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item {
    pub rule: usize,
    pub start: usize,
    pub next: usize,
}

impl Item {
    pub fn new(rule: usize, start: usize) -> Item {
        Item { rule: rule, next: 0, start: start }
    }

    pub fn advance(&self) -> Item {
        Item { rule: self.rule, next: self.next + 1, start: self.start }
    }

    pub fn render(&self, grammar: &Grammar) -> String {
        let rule = &grammar.rules[self.rule];
        let mut tokens: Vec<String> = rule.tokens.iter().map(|t| t.name()).collect();
        if self.next < tokens.len() {
            tokens.insert(self.next, "*".to_string());
        } else {
            tokens.push("*".to_string());
        }
        format!("{} -> {} ({})", rule.name, tokens.connect(" "), self.start)
    }
}

