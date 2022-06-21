use crate::token;

trait Node {
    fn token_literal(&self) -> &str;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

pub struct Program {
    statements: Vec<Box<dyn Statement>>,
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        if self.statements.is_empty() {
            return "";
        } else {
            return self.statements[0].token_literal();
        }
    }
}

struct LetStatement<'a> {
    token: token::Token,
    name: Identifier<'a>,
    value: dyn Expression,
}

impl Node for LetStatement<'_> {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Statement for LetStatement<'_> {
    fn statement_node(&self) { }
}


struct Identifier<'a> {
    token: token::Token,
    value: &'a str,
}

impl Node for Identifier<'_> {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Expression for Identifier<'_> {
    fn expression_node(&self) { }
}

