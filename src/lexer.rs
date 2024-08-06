pub struct Lexer;

impl Lexer {
    pub fn tokenize(content: &str) -> i32 {
        let mut status = 0;
        let mut line_number = 1;
        let mut chars = content.chars().peekable();

        while let Some(char) = chars.next() {
            match char {
                '\n' => line_number += 1,
                '\t' | ' ' => {},
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                '{' => println!("LEFT_BRACE {{ null"),
                '}' => println!("RIGHT_BRACE }} null"),
                '+' => println!("PLUS + null"),
                '-' => println!("MINUS - null"),
                '*' => println!("STAR * null"),
                ',' => println!("COMMA , null"),
                '.' => println!("DOT . null"),
                ';' => println!("SEMICOLON ; null"),
                '=' => {
                    if chars.peek() == Some(&'=') {
                        println!("EQUAL_EQUAL == null");
                        chars.next();
                    } else {
                        println!("EQUAL = null");
                    }
                },
                '!' => {
                    if chars.peek() == Some(&'=') {
                        println!("BANG_EQUAL != null");
                        chars.next();
                    } else {
                        println!("BANG ! null");
                    }
                },
                '<' => {
                    if chars.peek() == Some(&'=') {
                        println!("LESS_EQUAL <= null");
                        chars.next();
                    } else {
                        println!("LESS < null");
                    }
                },
                '>' => {
                    if chars.peek() == Some(&'=') {
                        println!("GREATER_EQUAL >= null");
                        chars.next();
                    } else {
                        println!("GREATER > null");
                    }
                },
                '/' => {
                    if chars.peek() == Some(&'/') {
                        while chars.next_if(|&c| c != '\n').is_some() {}
                    } else {
                        println!("SLASH / null");
                    }
                },
                '"' => {
                    let mut string = String::new();
                    let mut terminated = false;

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

                    if terminated {
                        println!("STRING \"{}\" {}", string, string);
                    } else {
                        status = 65;
                        eprintln!("[line {}] Error: Unterminated string.", line_number);
                    }
                },
                '0'..='9' => {
                    let mut number = char.to_string();
                    let mut has_dot = false;

                    while let Some(&c) = chars.peek() {
                        if c.is_numeric() {
                            number.push(c);
                            chars.next();
                        } else if c == '.' && !has_dot {
                            let mut lookahead = chars.clone();
                            lookahead.next(); // consume the dot
                            if let Some(next_char) = lookahead.next() {
                                if !next_char.is_numeric() { break; }
                                has_dot = true;
                                number.push(c);
                                chars.next();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    let parsed = number.parse::<f64>().unwrap();

                    if parsed.fract() == 0.0 {
                        println!("NUMBER {} {:.1}", number, parsed);
                    } else {
                        println!("NUMBER {} {}", number, parsed);
                    }
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut identifier = char.to_string();

                    while let Some(&c) = chars.peek() {
                        if !c.is_ascii_alphanumeric() && c != '_' { break }
                        identifier.push(c);
                        chars.next();
                    }

                    match identifier.as_str() {
                        "and" => println!("AND {} null", identifier),
                        "class" => println!("CLASS {} null", identifier),
                        "else" => println!("ELSE {} null", identifier),
                        "false" => println!("FALSE {} null", identifier),
                        "for" => println!("FOR {} null", identifier),
                        "fun" => println!("FUN {} null", identifier),
                        "if" => println!("IF {} null", identifier),
                        "nil" => println!("NIL {} null", identifier),
                        "or" => println!("OR {} null", identifier),
                        "print" => println!("PRINT {} null", identifier),
                        "return" => println!("RETURN {} null", identifier),
                        "super" => println!("SUPER {} null", identifier),
                        "this" => println!("THIS {} null", identifier),
                        "true" => println!("TRUE {} null", identifier),
                        "var" => println!("VAR {} null", identifier),
                        "while" => println!("WHILE {} null", identifier),
                        _ => println!("IDENTIFIER {} null", identifier)
                    }
                },
                _ => {
                    status = 65;
                    eprintln!("[line {}] Error: Unexpected character: {}", line_number, char);
                }
            }
        }

        println!("EOF  null");
        status
    }
}
