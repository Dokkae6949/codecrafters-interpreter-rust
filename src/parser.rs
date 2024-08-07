use std::{iter::Peekable, slice::Iter};

use crate::lexer::*;

pub struct Parser;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Literal(Token),
    Grouping(Box<Expression>),
    Unary(Token, Box<Expression>),
}

impl Parser {
    pub fn parse(tokens: &[Token]) -> Result<Expression, String> {
        let tokens = if tokens.last().map_or(false, |token| token.kind == TokenKind::Eof) {
            &tokens[0..tokens.len() - 1]
        } else {
            tokens
        };

        Self::unary(tokens.iter().peekable()).map(|(expr, _)| expr)
    }

    fn unary(mut iter: Peekable<Iter<'_, Token>>) -> Result<(Expression, Peekable<Iter<'_, Token>>), String> {
        match iter.next() {
            Some(token) => match &token.kind {
                TokenKind::Minus
                | TokenKind::Bang => {
                    let (rhs_expr, iter) = Self::unary(iter)?;
                    Ok((Expression::Unary(token.clone(), Box::new(rhs_expr)), iter))
                },
                _ => Self::primary(iter),
            },
            None => Err("Expected Expression".into())
        }
    }

    fn primary(mut iter: Peekable<Iter<'_, Token>>) -> Result<(Expression, Peekable<Iter<'_, Token>>), String> {
        match iter.next() {
            Some(token) => match &token.kind {
                TokenKind::LeftParen => {
                    let (expr, mut iter) = Self::unary(iter)?;
                    if let Some(Token { kind: TokenKind::RightParen, .. }) = iter.next() {
                        Ok((Expression::Grouping(Box::new(expr)), iter))
                    } else {
                        Err("Expected ')' after expression".into())
                    }
                },
                TokenKind::Keyword(Keyword::Nil)
                | TokenKind::Keyword(Keyword::True)
                | TokenKind::Keyword(Keyword::False)
                | TokenKind::String(_)
                | TokenKind::Number(_) => Ok((Expression::Literal(token.clone()), iter)),
                _ => Err("Unexpected token".into())
            },
            None => Err("Expected expression".into()),
        }
    }
}
