use super::token::{Token, TokenType};

pub struct Lexer<'a> {
  input: &'a str
}

impl<'a> Lexer<'a> {
  fn next_token(&self) -> Token {
    return Token { token_type: TokenType::EOF, literal: ""};
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_next_token() {
    let input = "=+(){},;";

    let tests = vec![
      (TokenType::ASSIGN, "="),
      (TokenType::PLUS, "+"),
      (TokenType::LPAREN, "("),
      (TokenType::RPAREN, ")"),
      (TokenType::LBRACE, "{"),
      (TokenType::RBRACE, "}"),
      (TokenType::COMMA, ","),
      (TokenType::SEMICOLON, ";"),
      (TokenType::EOF, "")
    ];


    let l = Lexer { input: input };
    for i in 0..tests.len() {
      let tok = l.next_token();
      let (expected_type, expected_literal) = &tests[i];
      assert_eq!(tok.token_type, *expected_type);
      assert_eq!(tok.literal, *expected_literal);
    }
  }
}
