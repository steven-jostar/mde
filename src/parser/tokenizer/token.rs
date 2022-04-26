use crate::parser::core::{Token, TokenType, BoxedTokenType};

/// The types of tokens
/// They used to divide symbol and char to simplify resolving of token and lex convertion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RawTokenType {
  /// i.e. symbols what need me to care about
  Symbol,
  /// i.e. normal chars
  Char,
}
impl TokenType for RawTokenType {}

#[derive(Debug, PartialEq)]
pub struct RawToken {
  pub value: char,
  pub typ: RawTokenType,
}

impl RawToken {
  pub fn new(value: char, typ: RawTokenType) -> Self {
    RawToken {
      value,
      typ,
    }
  }
}

impl Token for RawToken {
  fn raw(&self) -> char {
    self.value.clone()
  }
  fn typ(&self) -> BoxedTokenType {
    Box::new(self.typ.clone())
  }
}
