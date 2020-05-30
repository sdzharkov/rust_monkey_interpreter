use token::{Token, TokenType, Tokenizer};

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

  let tokenizer = Tokenizer::new(input.to_string());

  for test in &tests {
    let next_token = tokenizer.next_token().token;
    assert_eq!(test.token, next_token);
  }
}
