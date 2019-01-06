#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
  ILLEGAL,
  EOF,

  IDENT,
  INT,

  ASSIGN,
  PLUS,
  MINUS,
  BANG,
  ASTERISK,
  SLASH,

  LT,
  GT,
  
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

pub fn lookup_ident(ident: &str) -> TokenType {
  return match ident {
    "let" => TokenType::LET,
    "fn" => TokenType::FUNCTION,
    _ => TokenType::IDENT
  }
}

impl<'a> Token<'a> {
  pub fn new(token_type: TokenType, literal: &'a str) -> Token<'a> {
    return Token { token_type: token_type, literal: literal };
  }
}