use std::u16;
use crate::token_type::TokenType;
use crate::traits::Serialise;

pub struct Token {
  token: TokenType,
  lexeme: &'static str,
  literal: &'static str,
  line: u16
}
 
impl Serialise for Token {
  fn to_string(&self) -> &str {
    // return type + " " + lexeme + " " + literal;
    return "";
  }
}