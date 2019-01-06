use super::token::{self, Token, TokenType};

pub struct Lexer<'a> {
  input: &'a str,
  chars: std::str::Chars<'a>,
  position: usize,
  read_position: usize,
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
    if self.read_position >= self.input.len() {
      self.ch = '\0';
    } else {
      self.ch = self.chars.next().unwrap();
    }
    self.position = self.read_position;
    self.read_position += 1;
  }

  fn next_token(&mut self) -> Token<'a> {
    self.skip_white_space();
    let tok = match self.ch {
      '=' => Token::new(TokenType::ASSIGN, "="),
      '+' => Token::new(TokenType::PLUS, "+"),
      '-' => Token::new(TokenType::MINUS, "-"),
      '!' => Token::new(TokenType::BANG, "!"),
      '*' => Token::new(TokenType::ASTERISK, "*"),
      '/' => Token::new(TokenType::SLASH, "/"),
      '<' => Token::new(TokenType::LT, "<"),
      '>' => Token::new(TokenType::GT, ">"),
      ',' => Token::new(TokenType::COMMA, ","),
      ';' => Token::new(TokenType::SEMICOLON, ";"),
      '(' => Token::new(TokenType::LPAREN, "("),
      ')' => Token::new(TokenType::RPAREN, ")"),
      '{' => Token::new(TokenType::LBRACE, "{"),
      '}' => Token::new(TokenType::RBRACE, "}"),
      '\0' => Token::new(TokenType::EOF, ""),
      _ => {
        if self.ch.is_alphabetic() {
          let ident = self.read_identifier();
          return Token::new(token::lookup_ident(ident), ident);
        } else if self.ch.is_ascii_digit() {
          let number = self.read_number();
          return Token::new(TokenType::INT, number);
        } else {
          return Token::new(TokenType::ILLEGAL, &self.input[self.position..self.read_position]);
        }
      }
    };
    self.read_char();
    return tok;
  }

  fn read_number(&mut self) -> &'a str {
    let start_position = self.position;
    while self.ch.is_ascii_digit() {
      self.read_char();
    }
    return &self.input[start_position..self.position];
  }

  fn read_identifier(&mut self) -> &'a str {
    let start_position = self.position;
    while self.ch.is_alphabetic() {
      self.read_char();
    }
    return &self.input[start_position..self.position];
  }

  fn skip_white_space(&mut self) {
    while self.ch.is_ascii_whitespace() {
      self.read_char();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_next_token() {
    let input = "let five = 5;
    let ten = 10;
    let add = fn(x, y) {
      x + y;
    };
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;";

    let tests = vec![
      (TokenType::LET, "let"),
      (TokenType::IDENT, "five"),
      (TokenType::ASSIGN, "="),
      (TokenType::INT, "5"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::LET, "let"),
      (TokenType::IDENT, "ten"),
      (TokenType::ASSIGN, "="),
      (TokenType::INT, "10"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::LET, "let"),
      (TokenType::IDENT, "add"),
      (TokenType::ASSIGN, "="),
      (TokenType::FUNCTION, "fn"),
      (TokenType::LPAREN, "("),
      (TokenType::IDENT, "x"),
      (TokenType::COMMA, ","),
      (TokenType::IDENT, "y"),
      (TokenType::RPAREN, ")"),
      (TokenType::LBRACE, "{"),
      (TokenType::IDENT, "x"),
      (TokenType::PLUS, "+"),
      (TokenType::IDENT, "y"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::RBRACE, "}"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::LET, "let"),
      (TokenType::IDENT, "result"),
      (TokenType::ASSIGN, "="),
      (TokenType::IDENT, "add"),
      (TokenType::LPAREN, "("),
      (TokenType::IDENT, "five"),
      (TokenType::COMMA, ","),
      (TokenType::IDENT, "ten"),
      (TokenType::RPAREN, ")"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::BANG, "!"),
      (TokenType::MINUS, "-"),
      (TokenType::SLASH, "/"),
      (TokenType::ASTERISK, "*"),
      (TokenType::INT, "5"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::INT, "5"),
      (TokenType::LT, "<"),
      (TokenType::INT, "10"),
      (TokenType::GT, ">"),
      (TokenType::INT, "5"),
      (TokenType::SEMICOLON, ";"),
      (TokenType::EOF, "")
    ];


    let mut l = Lexer::new(input);
    for i in 0..tests.len() {
      let tok = l.next_token();
      let (expected_type, expected_literal) = &tests[i];
      assert_eq!(tok.literal, *expected_literal);
      assert_eq!(tok.token_type, *expected_type);
    }
  }
}
