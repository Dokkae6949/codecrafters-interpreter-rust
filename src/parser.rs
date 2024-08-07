use std::{iter::Peekable, slice::Iter};

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
    Grouping(Box<Expression>),
}

impl Parser {
    pub fn parse(tokens: &[Token]) -> Result<Expression, String> {
        let tokens = if tokens.last().map_or(false, |token| token.kind == TokenKind::Eof) {
            &tokens[0..tokens.len() - 1]
        } else {
            tokens
        };

        Self::primary(tokens.iter().peekable()).map(|(expr, _)| expr)
    }

    fn primary(mut iter: Peekable<Iter<'_, Token>>) -> Result<(Expression, Peekable<Iter<'_, Token>>), String> {
        match iter.next() {
            Some(token) => match &token.kind {
                TokenKind::LeftParen => {
                    let (expr, mut iter) = Self::primary(iter)?;
                    if let Some(Token { kind: TokenKind::RightParen, .. }) = iter.next() {
                        Ok((Expression::Grouping(Box::new(expr)), iter))
                    } else {
                        Err("Expected ')' after expression".into())
                    }
                },
                TokenKind::Keyword(Keyword::Nil) => Ok((Expression::Literal(Literal::Nil), iter)),
                TokenKind::Keyword(Keyword::True) => Ok((Expression::Literal(Literal::Bool(true)), iter)),
                TokenKind::Keyword(Keyword::False) => Ok((Expression::Literal(Literal::Bool(false)), iter)),
                TokenKind::String(string) => Ok((Expression::Literal(Literal::String(string.to_string())), iter)),
                TokenKind::Number(number) => Ok((Expression::Literal(Literal::Number(*number)), iter)),
                _ => Err("Unexpected token".into())
            },
            None => Err("Expected expression".into()),
        }
    }
}
