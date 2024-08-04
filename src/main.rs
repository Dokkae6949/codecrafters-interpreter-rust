use std::env;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            tokenize(&file_contents);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}

pub fn tokenize(content: &str) {
    let mut line_number = 1;

    for char in content.chars() {
        match char {
            '(' => println!("LEFT_PAREN {} null", char),
            ')' => println!("RIGHT_PAREN {} null", char),
            '{' => println!("LEFT_BRACE {} null", char),
            '}' => println!("RIGHT_BRACE {} null", char),
            '+' => println!("PLUS {} null", char),
            '-' => println!("MINUS {} null", char),
            '*' => println!("STAR {} null", char),
            ',' => println!("COMMA {} null", char),
            '.' => println!("DOT {} null", char),
            ';' => println!("SEMICOLON {} null", char),
            '\n' => line_number += 1,
            _ => println!("[line {}] Error: Unexpected character: {}", line_number, char),
        };
    }

    println!("EOF  null");
}
