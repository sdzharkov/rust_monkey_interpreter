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
    pub fn lookup_ident(ident: String) -> Token {
        match ident.as_str() {
            "let" => Token {
                token: TokenType::LET,
                literal: ident,
            },
            "fn" => Token {
                token: TokenType::FUNCTION,
                literal: ident,
            },
            _ => Token {
                token: TokenType::IDENT(ident.clone()),
                literal: ident,
            },
        }
    }
}
