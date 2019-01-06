use std::io::{Result, Read, BufReader, Write};
use std::io::prelude::*;
use super::token::TokenType;
use super::lexer;

const PROMPT: &str = ">>";

pub fn start<R: Read, W: Write>(read: R, mut write: W) -> Result<()> {
  
  let mut reader = BufReader::new(read);
  loop {
    write!(write, "{} ", PROMPT)?;
    write.flush()?;
    let mut buf = String::new();
    let len = reader.read_line(&mut buf)?;
    if len <= 0 {
      return Ok(());
    }

    let mut lexer = lexer::Lexer::new(&buf);
    loop {
      let token = lexer.next_token();
      if token.token_type != TokenType::EOF {
        writeln!(write, "{:?} {}", token.token_type, token.literal)?;
      } else {
        break;
      }
    }
  }
}