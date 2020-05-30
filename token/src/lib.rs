#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// enum TokenType {
//     ILLEGAL,
//     EOF    ,
//     IDENT = "IDENT", // add, foobar, x, y, ...,
//     INT   = "INT",   // 1343456,
    
//     // Operators
//     ASSIGN   = "=",
//     PLUS     = "+",
//     // Delimiters
//     COMMA     = ",",
//     SEMICOLON = ";",
//     LPAREN = "(",
//     RPAREN = ")",
//     LBRACE = "{",
//     RBRACE = "}",
//     // Keywords
//     FUNCTION = "FUNCTION",
//     LET      = "LET",
// }

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

pub fn hello() {
    println!("Hello, world!");
}
  

pub struct Tokenizer {
    input: String,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        unimplemented!();
    }

    pub fn next_token(&self) -> Token {
        unimplemented!();
    } 
}
