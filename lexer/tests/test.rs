use lexer::Lexer;
use token::{Token, TokenType};

#[test]
fn test_simple_tokens() {
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
    },
  ];

  let mut lexer = Lexer::new(input.to_string());
  println!("lexer: {:?}", lexer);

  for test in &tests {
    let next_token = lexer.next_token();
    println!(
      "test: {:?}, next: {:?}, lexer: {:?}",
      test, next_token, lexer
    );
    assert_eq!(test.token, next_token.token);
    assert_eq!(test.literal, next_token.literal)
  }
}

#[test]
fn test_multiple_tokens() {
  let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
  ";

  let tests = [
    Token {
      token: TokenType::LET,
      literal: "let".to_string(),
    },
    Token {
      token: TokenType::IDENT("five".to_string()),
      literal: "five".to_string(),
    },
    Token {
      token: TokenType::ASSIGN,
      literal: "=".to_string(),
    },
    Token {
      token: TokenType::INT,
      literal: "5".to_string(),
    },
    Token {
      token: TokenType::SEMICOLON,
      literal: ";".to_string(),
    },
    Token {
      token: TokenType::LET,
      literal: "let".to_string(),
    },
    Token {
      token: TokenType::IDENT("ten".to_string()),
      literal: "ten".to_string(),
    },
    Token {
      token: TokenType::ASSIGN,
      literal: "=".to_string(),
    },
    Token {
      token: TokenType::INT,
      literal: "10".to_string(),
    },
    Token {
      token: TokenType::SEMICOLON,
      literal: ";".to_string(),
    },
    Token {
      token: TokenType::LET,
      literal: "let".to_string(),
    },
    Token {
      token: TokenType::IDENT("add".to_string()),
      literal: "add".to_string(),
    },
    Token {
      token: TokenType::ASSIGN,
      literal: "=".to_string(),
    },
    Token {
      token: TokenType::FUNCTION,
      literal: "fn".to_string(),
    },
    Token {
      token: TokenType::LPAREN,
      literal: "(".to_string(),
    },
    Token {
      token: TokenType::IDENT("x".to_string()),
      literal: "x".to_string(),
    },
    Token {
      token: TokenType::COMMA,
      literal: ",".to_string(),
    },
    Token {
      token: TokenType::IDENT("y".to_string()),
      literal: "y".to_string(),
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
      token: TokenType::IDENT("x".to_string()),
      literal: "x".to_string(),
    },
    Token {
      token: TokenType::PLUS,
      literal: "+".to_string(),
    },
    Token {
      token: TokenType::IDENT("y".to_string()),
      literal: "y".to_string(),
    },
    Token {
      token: TokenType::SEMICOLON,
      literal: ";".to_string(),
    },
    Token {
      token: TokenType::RBRACE,
      literal: "}".to_string(),
    },
    Token {
      token: TokenType::SEMICOLON,
      literal: ";".to_string(),
    },
    Token {
      token: TokenType::LET,
      literal: "let".to_string(),
    },
    Token {
      token: TokenType::IDENT("result".to_string()),
      literal: "result".to_string(),
    },
    Token {
      token: TokenType::ASSIGN,
      literal: "=".to_string(),
    },
    Token {
      token: TokenType::IDENT("add".to_string()),
      literal: "add".to_string(),
    },
    Token {
      token: TokenType::LPAREN,
      literal: "(".to_string(),
    },
    Token {
      token: TokenType::IDENT("five".to_string()),
      literal: "five".to_string(),
    },
    Token {
      token: TokenType::COMMA,
      literal: ",".to_string(),
    },
    Token {
      token: TokenType::IDENT("ten".to_string()),
      literal: "ten".to_string(),
    },
    Token {
      token: TokenType::RPAREN,
      literal: ")".to_string(),
    },
    Token {
      token: TokenType::SEMICOLON,
      literal: ";".to_string(),
    },
  ];

  let mut lexer = Lexer::new(input.to_string());
  println!("lexer: {:?}", lexer);

  for (_index, test) in tests.iter().enumerate() {
    let next_token = lexer.next_token();
    println!(
      "test: {:?}, next: {:?}, lexer: {:?}",
      test, next_token, lexer
    );
    assert_eq!(test.token, next_token.token);
    assert_eq!(test.literal, next_token.literal)
  }
}
