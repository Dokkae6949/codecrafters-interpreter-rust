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

            match Lexer::tokenize(&file_contents) {
                Ok(_) => exit(0),
                Err(status) => exit(status)
            };
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
