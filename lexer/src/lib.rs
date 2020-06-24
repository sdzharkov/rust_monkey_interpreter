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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token: Token = match &self.ch {
            '=' => Token {
                token: TokenType::ASSIGN,
                literal: self.ch.to_string(),
            },
            ';' => Token {
                token: TokenType::SEMICOLON,
                literal: self.ch.to_string(),
            },
            '(' => Token {
                token: TokenType::LPAREN,
                literal: self.ch.to_string(),
            },
            ')' => Token {
                token: TokenType::RPAREN,
                literal: self.ch.to_string(),
            },
            ',' => Token {
                token: TokenType::COMMA,
                literal: self.ch.to_string(),
            },
            '+' => Token {
                token: TokenType::PLUS,
                literal: self.ch.to_string(),
            },
            '{' => Token {
                token: TokenType::LBRACE,
                literal: self.ch.to_string(),
            },
            '}' => Token {
                token: TokenType::RBRACE,
                literal: self.ch.to_string(),
            },
            '0' => Token {
                token: TokenType::EOF,
                literal: self.ch.to_string(),
            },
            _ => {
                if is_letter(&self.ch) {
                    let identifier = self.read_identifier();
                    Token::lookup_ident(identifier)
                } else if self.ch.is_numeric() {
                    let num = self.read_number();
                    Token {
                        token: TokenType::INT,
                        literal: num,
                    }
                } else {
                    Token {
                        token: TokenType::ILLEGAL,
                        literal: self.ch.to_string(),
                    }
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
