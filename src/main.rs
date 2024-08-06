mod lexer;
mod parser;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

use lexer::*;
use parser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize|evaluate <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let tokens = match Lexer::tokenize(&file_contents) {
                Ok(tokens) => tokens,
                Err(status) => exit(status)
            };

            for token in tokens.iter() {
                match &token.kind {
                    TokenKind::Eof => println!("EOF  null"),
                    TokenKind::LeftParen => println!("LEFT_PAREN ( null"),
                    TokenKind::RightParen => println!("RIGHT_PAREN ) null"),
                    TokenKind::LeftBrace => println!("LEFT_BRACE {{ null"),
                    TokenKind::RightBrace => println!("RIGHT_BRACE }} null"),
                    TokenKind::Plus => println!("PLUS + null"),
                    TokenKind::Minus => println!("MINUS - null"),
                    TokenKind::Star => println!("STAR * null"),
                    TokenKind::Comma => println!("COMMA , null"),
                    TokenKind::Dot => println!("DOT . null"),
                    TokenKind::Semicolon => println!("SEMICOLON ; null"),
                    TokenKind::Equal => println!("EQUAL = null"),
                    TokenKind::EqualEqual => println!("EQUAL_EQUAL == null"),
                    TokenKind::Bang => println!("BANG ! null"),
                    TokenKind::BangEqual => println!("BANG_EQUAL != null"),
                    TokenKind::Less => println!("LESS < null"),
                    TokenKind::LessEqual => println!("LESS_EQUAL <= null"),
                    TokenKind::Greater => println!("GREATER > null"),
                    TokenKind::GreaterEqual => println!("GREATER_EQUAL >= null"),
                    TokenKind::Slash => println!("SLASH / null"),
                    TokenKind::String(s) => println!("STRING \"{}\" {}", s, s),
                    TokenKind::Number(n) => {
                        if n.fract() == 0.0 {
                            println!("NUMBER {} {:.1}", n, n)
                        } else {
                            println!("NUMBER {} {}", n, n)
                        }
                    }
                    TokenKind::Identifier(s) => println!("IDENTIFIER {} null", s),
                    TokenKind::Keyword(keyword) => {
                        match keyword {
                            Keyword::Nil => println!("NIL nil null"),
                            Keyword::And => println!("AND and null"),
                            Keyword::Class => println!("CLASS class null"),
                            Keyword::Else => println!("ELSE else null"),
                            Keyword::False => println!("FALSE false null"),
                            Keyword::For => println!("FOR for null"),
                            Keyword::Fun => println!("FUN fun null"),
                            Keyword::If => println!("IF if null"),
                            Keyword::Or => println!("OR or null"),
                            Keyword::Print => println!("PRINT print null"),
                            Keyword::Return => println!("RETURN return null"),
                            Keyword::Super => println!("SUPER super null"),
                            Keyword::This => println!("THIS this null"),
                            Keyword::True => println!("TRUE true null"),
                            Keyword::Var => println!("VAR var null"),
                            Keyword::While => println!("WHILE while null"),
                        }
                    }
                }
            }
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let tokens = match Lexer::tokenize(&file_contents) {
                Ok(tokens) => tokens,
                Err(status) => exit(status)
            };

            let expression = match Parser::parse(tokens) {
                Some(expr) => expr,
                None => exit(1)
            };

            match expression {
                Expression::Literal(literal) => match literal {
                    Literal::Nil => println!("nil"),
                    Literal::Bool(value) => println!("{}", value),
                },
            };
        },
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
