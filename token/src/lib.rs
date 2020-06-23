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
}
