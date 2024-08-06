pub struct Lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Eof,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Star,
    Comma,
    Dot,
    Semicolon,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Slash,
    String(String),
    Number(f64),
    Identifier(String),
    Keyword(Keyword)
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String) -> Self {
        Self { kind, lexeme }
    }
}

impl Lexer {
    pub fn tokenize(content: &str) -> Result<Vec<Token>, i32> {
        let mut status = 0;
        let mut line_number = 1;
        let mut chars = content.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(char) = chars.next() {
            match char {
                '\n' => line_number += 1,
                '\t' | ' ' => {},
                '(' => tokens.push(Token::new(TokenKind::LeftParen, char.to_string())),
                ')' => tokens.push(Token::new(TokenKind::RightParen, char.to_string())),
                '{' => tokens.push(Token::new(TokenKind::LeftBrace, char.to_string())),
                '}' => tokens.push(Token::new(TokenKind::RightBrace, char.to_string())),
                '+' => tokens.push(Token::new(TokenKind::Plus, char.to_string())),
                '-' => tokens.push(Token::new(TokenKind::Minus, char.to_string())),
                '*' => tokens.push(Token::new(TokenKind::Star, char.to_string())),
                ',' => tokens.push(Token::new(TokenKind::Comma, char.to_string())),
                '.' => tokens.push(Token::new(TokenKind::Dot, char.to_string())),
                ';' => tokens.push(Token::new(TokenKind::Semicolon, char.to_string())),
                '=' => {
                    if chars.peek() == Some(&'=') {
                        tokens.push(Token::new(TokenKind::EqualEqual, "==".to_string()));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Equal, char.to_string()));
                    }
                },
                '!' => {
                    if chars.peek() == Some(&'=') {
                        tokens.push(Token::new(TokenKind::BangEqual, "!=".to_string()));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Bang, char.to_string()));
                    }
                },
                '<' => {
                    if chars.peek() == Some(&'=') {
                        tokens.push(Token::new(TokenKind::LessEqual, "<=".to_string()));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Less, char.to_string()));
                    }
                },
                '>' => {
                    if chars.peek() == Some(&'=') {
                        tokens.push(Token::new(TokenKind::GreaterEqual, ">=".to_string()));
                        chars.next();
                    } else {
                        tokens.push(Token::new(TokenKind::Greater, char.to_string()));
                    }
                },
                '/' => {
                    if chars.peek() == Some(&'/') {
                        while chars.next_if(|&c| c != '\n').is_some() {}
                    } else {
                        tokens.push(Token::new(TokenKind::Slash, char.to_string()));
                    }
                }
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
                        tokens.push(Token::new(TokenKind::String(string.clone()), string));
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
                    tokens.push(Token::new(TokenKind::Number(parsed), number));
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut identifier = char.to_string();

                    while let Some(&c) = chars.peek() {
                        if !c.is_ascii_alphanumeric() && c != '_' { break }
                        identifier.push(c);
                        chars.next();
                    }

                   let kind = match identifier.as_str() {
                        "and" => TokenKind::Keyword(Keyword::And),
                        "class" => TokenKind::Keyword(Keyword::Class),
                        "else" => TokenKind::Keyword(Keyword::Else),
                        "false" => TokenKind::Keyword(Keyword::False),
                        "for" => TokenKind::Keyword(Keyword::For),
                        "fun" => TokenKind::Keyword(Keyword::Fun),
                        "if" => TokenKind::Keyword(Keyword::If),
                        "nil" => TokenKind::Keyword(Keyword::Nil),
                        "or" => TokenKind::Keyword(Keyword::Or),
                        "print" => TokenKind::Keyword(Keyword::Print),
                        "return" => TokenKind::Keyword(Keyword::Return),
                        "super" => TokenKind::Keyword(Keyword::Super),
                        "this" => TokenKind::Keyword(Keyword::This),
                        "true" => TokenKind::Keyword(Keyword::True),
                        "var" => TokenKind::Keyword(Keyword::Var),
                        "while" => TokenKind::Keyword(Keyword::While),
                        _ => TokenKind::Identifier(identifier.clone()),
                    };

                   tokens.push(Token::new(kind, identifier));
                },
                _ => {
                    status = 65;
                    eprintln!("[line {}] Error: Unexpected character: {}", line_number, char);
                }
            }
        }

        tokens.push(Token::new(TokenKind::Eof, "".to_string()));

        if status == 0 {
            return Ok(tokens)
        } else {
            Err(status)
        }
    }
}
