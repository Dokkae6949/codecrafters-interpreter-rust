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

            let tokens = Lexer::tokenize(&file_contents);
            let mut status = 0;

            for token in tokens.iter() {
                match &token.kind {
                    TokenKind::Eof => println!("EOF  null"),
                    TokenKind::Error(msg, status_code) => { eprintln!("{}", msg); status = *status_code; },
                    TokenKind::LeftParen => println!("LEFT_PAREN {} null", token.lexeme),
                    TokenKind::RightParen => println!("RIGHT_PAREN {} null", token.lexeme),
                    TokenKind::LeftBrace => println!("LEFT_BRACE {} null", token.lexeme),
                    TokenKind::RightBrace => println!("RIGHT_BRACE {} null", token.lexeme),
                    TokenKind::Plus => println!("PLUS {} null", token.lexeme),
                    TokenKind::Minus => println!("MINUS {} null", token.lexeme),
                    TokenKind::Star => println!("STAR {} null", token.lexeme),
                    TokenKind::Comma => println!("COMMA {} null", token.lexeme),
                    TokenKind::Dot => println!("DOT {} null", token.lexeme),
                    TokenKind::Semicolon => println!("SEMICOLON {} null", token.lexeme),
                    TokenKind::Equal => println!("EQUAL {} null", token.lexeme),
                    TokenKind::EqualEqual => println!("EQUAL_EQUAL {} null", token.lexeme),
                    TokenKind::Bang => println!("BANG {} null", token.lexeme),
                    TokenKind::BangEqual => println!("BANG_EQUAL {} null", token.lexeme),
                    TokenKind::Less => println!("LESS {} null", token.lexeme),
                    TokenKind::LessEqual => println!("LESS_EQUAL {} null", token.lexeme),
                    TokenKind::Greater => println!("GREATER {} null", token.lexeme),
                    TokenKind::GreaterEqual => println!("GREATER_EQUAL {} null", token.lexeme),
                    TokenKind::Slash => println!("SLASH {} null", token.lexeme),
                    TokenKind::String(s) => println!("STRING \"{}\" {}", token.lexeme, s),
                    TokenKind::Number(n) => {
                        if n.fract() == 0.0 {
                            println!("NUMBER {} {:.1}", token.lexeme, n)
                        } else {
                            println!("NUMBER {} {}", token.lexeme, n)
                        }
                    }
                    TokenKind::Identifier(s) => println!("IDENTIFIER {} null", s),
                    TokenKind::Keyword(keyword) => {
                        match keyword {
                            Keyword::Nil => println!("NIL {} null", token.lexeme),
                            Keyword::And => println!("AND {} null", token.lexeme),
                            Keyword::Class => println!("CLASS {} null", token.lexeme),
                            Keyword::Else => println!("ELSE {} null", token.lexeme),
                            Keyword::False => println!("FALSE {} null", token.lexeme),
                            Keyword::For => println!("FOR {} null", token.lexeme),
                            Keyword::Fun => println!("FUN {} null", token.lexeme),
                            Keyword::If => println!("IF {} null", token.lexeme),
                            Keyword::Or => println!("OR {} null", token.lexeme),
                            Keyword::Print => println!("PRINT {} null", token.lexeme),
                            Keyword::Return => println!("RETURN {} null", token.lexeme),
                            Keyword::Super => println!("SUPER {} null", token.lexeme),
                            Keyword::This => println!("THIS {} null", token.lexeme),
                            Keyword::True => println!("TRUE {} null", token.lexeme),
                            Keyword::Var => println!("VAR {} null", token.lexeme),
                            Keyword::While => println!("WHILE {} null", token.lexeme),
                        }
                    }
                }
            }

            exit(status);
        }
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            let tokens = Lexer::tokenize(&file_contents);
            let expression = match Parser::parse(&tokens) {
                Ok(expr) => expr,
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1);
                }
            };
            
            print_expression(&expression);
        },
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

fn print_expression(expression: &Expression) {
    match expression {
        Expression::Literal(Literal::Nil) => println!("nil"),
        Expression::Literal(Literal::Bool(bool)) => println!("{}", bool),
        Expression::Literal(Literal::String(str)) => println!("{}", str),
        Expression::Literal(Literal::Number(num)) => println!("{}", num),
        Expression::Grouping(expr) => print_expression(expr),
    };
}
