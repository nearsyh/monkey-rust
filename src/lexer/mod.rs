use super::token::{Token, TokenType};

pub struct Lexer<'a> {
  input: &'a str,
  chars: std::str::Chars<'a>,
  position: u32,
  read_position: u32,
  ch: char
}

impl<'a> Lexer<'a> {
  fn new(input: &'a str) -> Lexer<'a> {
    let mut ret = Lexer {
      input: input, 
      chars: input.chars(), 
      position: 0, 
      read_position: 0,
      ch: '\0' };
    ret.read_char();
    return ret;
  }

  fn read_char(&mut self) {
    if self.read_position >= (self.input.len() as u32) {
      self.ch = '\0';
    } else {
      self.ch = self.chars.next().unwrap();
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  fn next_token(&mut self) -> Token<'a> {
    let tok = match self.ch {
      '=' => Token::new(TokenType::ASSIGN, "="),
      '+' => Token::new(TokenType::PLUS, "+"),
      ',' => Token::new(TokenType::COMMA, ","),
      ';' => Token::new(TokenType::SEMICOLON, ";"),
      '(' => Token::new(TokenType::LPAREN, "("),
      ')' => Token::new(TokenType::RPAREN, ")"),
      '{' => Token::new(TokenType::LBRACE, "{"),
      '}' => Token::new(TokenType::RBRACE, "}"),
      '\0' => Token::new(TokenType::EOF, ""),
      _ => Token::new(TokenType::ILLEGAL, "")
    };
    self.read_char();
    return tok;
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


    let mut l = Lexer::new(input);
    for i in 0..tests.len() {
      let tok = l.next_token();
      let (expected_type, expected_literal) = &tests[i];
      assert_eq!(tok.token_type, *expected_type);
      assert_eq!(tok.literal, *expected_literal);
    }
  }
}
