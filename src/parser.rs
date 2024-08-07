use crate::lexer::*;

pub struct Parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Nil,
    Bool(bool),
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Literal),
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Option<Expression> {
        match &tokens.first().unwrap().kind {
            TokenKind::Keyword(Keyword::Nil) => Some(Expression::Literal(Literal::Nil)),
            TokenKind::Keyword(Keyword::True) => Some(Expression::Literal(Literal::Bool(true))),
            TokenKind::Keyword(Keyword::False) => Some(Expression::Literal(Literal::Bool(false))),
            TokenKind::String(string) => Some(Expression::Literal(Literal::String(string.to_string()))),
            TokenKind::Number(number) => Some(Expression::Literal(Literal::Number(*number))),
            _ => None
        }
    }
}
