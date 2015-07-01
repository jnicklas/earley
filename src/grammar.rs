use token::Token;

#[derive(Debug)]
pub struct Grammar {
    pub starting_rule: &'static str,
    pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
    pub name: &'static str,
    pub tokens: Vec<Token>,
}
