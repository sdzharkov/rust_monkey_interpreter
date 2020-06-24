#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT,
    // Operators
    ASSIGN,
    PLUS,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // Keywords
    FUNCTION,
    LET,
}
  
#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token: TokenType,
    pub literal: String,
}

impl Token {
    pub fn next_token(character: &char) -> Token {
        match character {
            '=' => Token { token: TokenType::ASSIGN, literal: character.to_string() },
            ';' => Token { token: TokenType::SEMICOLON, literal: character.to_string() },
            '(' => Token { token: TokenType::LPAREN, literal: character.to_string() },
            ')' => Token { token: TokenType::RPAREN, literal: character.to_string() },
            ',' => Token { token: TokenType::COMMA, literal: character.to_string() },
            '+' => Token { token: TokenType::PLUS, literal: character.to_string() },
            '{' => Token { token: TokenType::LBRACE, literal: character.to_string() },
            '}' => Token { token: TokenType::RBRACE, literal: character.to_string() },
            '0' => Token { token: TokenType::EOF, literal: character.to_string() },
            _ => Token { token: TokenType::ILLEGAL, literal: character.to_string() }
        }
    }

    pub fn next_identifier_token(ident: String) -> Token {
        match ident.as_str() {
            "let" => Token { token: TokenType::LET, literal: ident },
            _ => Token { token: TokenType::IDENT(ident.clone()), literal: ident }
        }
    }
}
