#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
  ILLEGAL,
  EOF,

  IDENT,
  INT,

  ASSIGN,
  PLUS,
  
  COMMA,
  SEMICOLON,

  LPAREN,
  RPAREN,
  LBRACE,
  RBRACE,

  FUNCTION,
  LET
}

pub struct Token<'a> {
  pub token_type: TokenType,
  pub literal: &'a str
}