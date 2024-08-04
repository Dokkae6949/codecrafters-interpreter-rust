pub struct Lexer;

impl Lexer {
    pub fn tokenize(content: &str) -> i32 {
        let mut status = 0;
        let mut line_number = 1;
        let mut chars = content.chars().peekable();

        while let Some(char) = chars.next() {
            match char {
                '\n' => line_number += 1,
                '\t' => { },
                ' ' => { },
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
                '=' => {
                    if chars.peek() == Some(&'=') {
                        println!("EQUAL_EQUAL {}{} null", char, chars.next().unwrap());
                    } else {
                        println!("EQUAL {} null", char);
                    }
                },
                '!' => {
                    if chars.peek() == Some(&'=') {
                        println!("BANG_EQUAL {}{} null", char, chars.next().unwrap());
                    } else {
                        println!("BANG {} null", char);
                    }
                },
                '<' => {
                    if chars.peek() == Some(&'=') {
                        println!("LESS_EQUAL {}{} null", char, chars.next().unwrap());
                    } else {
                        println!("LESS {} null", char);
                    }
                },
                '>' => {
                    if chars.peek() == Some(&'=') {
                        println!("GREATER_EQUAL {}{} null", char, chars.next().unwrap());
                    } else {
                        println!("GREATER {} null", char);
                    }
                },
                '/' => {
                    if chars.peek() == Some(&'/') {
                        while chars.peek() != Some(&'\n') && chars.peek() != None {
                            chars.next();
                        }
                    } else {
                        println!("SLASH {} null", char);
                    }
                },
                '"' => {
                    let mut terminated = false;
                    let mut string = String::new();

                    while let Some(c) = chars.next() {
                        if c == '"' {
                            terminated = true;
                            break;
                        }

                        if c == '\n' {
                            line_number += 1;
                        }

                        string.push(c);
                    }

                    if !terminated {
                        status = 65;
                        eprintln!("[line {}] Error: Unterminated string.", line_number);
                    } else {
                        println!("STRING \"{}\" {}", string, string);
                    }
                },
                _ => {
                    status = 65;
                    eprintln!("[line {}] Error: Unexpected character: {}", line_number, char);
                },
            };
        }

        println!("EOF  null");
        status
    }
}
