pub struct Lexer;

impl Lexer {
    pub fn tokenize(content: &str) -> i32 {
        let mut status = 0;
        let mut line_number = 1;
        let mut chars = content.chars().peekable();

        while let Some(char) = chars.next() {
            match char {
                '\n' => line_number += 1,
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
                        println!("EQUAL_EQUAL == null");
                        chars.next();
                    } else {
                        println!("EQUAL {} null", char);
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
