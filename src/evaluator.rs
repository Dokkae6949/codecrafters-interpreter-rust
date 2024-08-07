use crate::{Expression, Keyword, Token, TokenKind};

pub struct Evaluator;

impl Evaluator {
    pub fn evaluate(expression: &Expression) -> Result<Expression, String> {
        match expression {
            Expression::Literal(_) => Ok(expression.clone()),
            Expression::Grouping(expr) => Self::evaluate(&expr),
            Expression::Unary(token, expr) => {
                let result = Self::evaluate(&expr)?;
                match token.kind {
                    TokenKind::Minus => {
                        if let Expression::Literal(Token {kind: TokenKind::Number(number), lexeme}) = result {
                            Ok(Expression::Literal(Token::new(TokenKind::Number(-number), lexeme)))
                        } else {
                            Err("Cannot negate non-number types".into())
                        }
                    },
                    TokenKind::Bang => {
                        if let Expression::Literal(Token {kind: TokenKind::Number(_), lexeme}) = result {
                            Ok(Expression::Literal(Token::new(TokenKind::Keyword(Keyword::False), lexeme)))
                        } else if let Expression::Literal(Token {kind: TokenKind::Keyword(Keyword::False), lexeme}) = result {
                            Ok(Expression::Literal(Token::new(TokenKind::Keyword(Keyword::True), lexeme)))
                        } else if let Expression::Literal(Token {kind: TokenKind::Keyword(Keyword::True), lexeme}) = result {
                            Ok(Expression::Literal(Token::new(TokenKind::Keyword(Keyword::False), lexeme)))
                        } else {
                            Err("Cannot negate non-number types".into())
                        }
                    },
                    _ => Err("Unsupported unary operator".into()),
                }
            },
        }
    }
}
