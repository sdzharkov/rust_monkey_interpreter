#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT(String),
    INT,
    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOTEQ,
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
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
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
            "true" => Token {
                token: TokenType::TRUE,
                literal: ident,
            },
            "false" => Token {
                token: TokenType::FALSE,
                literal: ident,
            },
            "if" => Token {
                token: TokenType::IF,
                literal: ident,
            },
            "else" => Token {
                token: TokenType::ELSE,
                literal: ident,
            },
            "return" => Token {
                token: TokenType::RETURN,
                literal: ident,
            },
            _ => Token {
                token: TokenType::IDENT(ident.clone()),
                literal: ident,
            },
        }
    }
}
