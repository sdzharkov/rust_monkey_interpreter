use token::{Token, TokenType};
use lexer::{Lexer};

#[test]
fn test_next_token() {
  let input = "=+(){},;";

  let tests = [
    Token {
      token: TokenType::ASSIGN,
      literal: "=".to_string(),
    },
    Token {
      token: TokenType::PLUS,
      literal: "+".to_string(),
    },
    Token {
      token: TokenType::LPAREN,
      literal: "(".to_string(),
    },
    Token {
      token: TokenType::RPAREN,
      literal: ")".to_string(),
    },
    Token {
      token: TokenType::LBRACE,
      literal: "{".to_string(),
    },
    Token {
      token: TokenType::RBRACE,
      literal: "}".to_string(),
    },
    Token {
      token: TokenType::COMMA,
      literal: ",".to_string(),
    },
    Token {
      token: TokenType::SEMICOLON,
      literal: ";".to_string(),
    }
  ];

  let mut lexer = Lexer::new(input.to_string());
  println!("lexer: {:?}", lexer);

  for test in &tests {
    let next_token = lexer.next_token();
    println!("test: {:?}, next: {:?}, lexer: {:?}", test, next_token, lexer);
    assert_eq!(test.token, next_token.token);
    assert_eq!(test.literal, next_token.literal)
  }
}
