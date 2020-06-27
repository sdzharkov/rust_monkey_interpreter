use token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            ch: input.chars().next().unwrap(),
            input,
            position: 0,
            read_position: 0,
        };
        lexer.read_char();

        lexer
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;

        while is_letter(&self.ch) {
            self.read_char();
        }

        self.input[position..self.position].to_string()
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            // @TODO: use alternative rescueing
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;

        while self.ch.is_numeric() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    pub fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        }

        return self.input.chars().nth(self.read_position).unwrap();
    }

    // @TODO: Refactor to make this cleaner
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token: Token;
        match &self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token {
                        token: TokenType::EQ,
                        literal: "==".to_string(),
                    }
                } else {
                    token = Token {
                        token: TokenType::ASSIGN,
                        literal: self.ch.to_string(),
                    }
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    token = Token {
                        token: TokenType::NOTEQ,
                        literal: "!=".to_string(),
                    }
                } else {
                    token = Token {
                        token: TokenType::BANG,
                        literal: self.ch.to_string(),
                    }
                }
            }
            ';' => {
                token = Token {
                    token: TokenType::SEMICOLON,
                    literal: self.ch.to_string(),
                }
            }
            '(' => {
                token = Token {
                    token: TokenType::LPAREN,
                    literal: self.ch.to_string(),
                }
            }
            ')' => {
                token = Token {
                    token: TokenType::RPAREN,
                    literal: self.ch.to_string(),
                }
            }
            ',' => {
                token = Token {
                    token: TokenType::COMMA,
                    literal: self.ch.to_string(),
                }
            }
            '+' => {
                token = Token {
                    token: TokenType::PLUS,
                    literal: self.ch.to_string(),
                }
            }
            '-' => {
                token = Token {
                    token: TokenType::MINUS,
                    literal: self.ch.to_string(),
                }
            }
            '*' => {
                token = Token {
                    token: TokenType::ASTERISK,
                    literal: self.ch.to_string(),
                }
            }
            '/' => {
                token = Token {
                    token: TokenType::SLASH,
                    literal: self.ch.to_string(),
                }
            }
            '<' => {
                token = Token {
                    token: TokenType::LT,
                    literal: self.ch.to_string(),
                }
            }
            '>' => {
                token = Token {
                    token: TokenType::GT,
                    literal: self.ch.to_string(),
                }
            }
            '{' => {
                token = Token {
                    token: TokenType::LBRACE,
                    literal: self.ch.to_string(),
                }
            }
            '}' => {
                token = Token {
                    token: TokenType::RBRACE,
                    literal: self.ch.to_string(),
                }
            }
            // '\0' => {
            //     token = Token {
            //         token: TokenType::EOF,
            //         literal: self.ch.to_string(),
            //     }
            // }
            _ => {
                if is_letter(&self.ch) {
                    let identifier = self.read_identifier();
                    return Token::lookup_ident(identifier);
                } else if self.ch.is_numeric() {
                    let num = self.read_number();
                    return Token {
                        token: TokenType::INT,
                        literal: num,
                    };
                } else {
                    return Token {
                        token: TokenType::ILLEGAL,
                        literal: self.ch.to_string(),
                    };
                }
            }
        };

        self.read_char();
        token
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }
}

fn is_letter(character: &char) -> bool {
    character.is_alphabetic() || *character == '_'
}
