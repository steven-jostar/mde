use crate::parser::{
  Parser,
  ParserResult,
  ResultData,
};

pub struct Next;
impl Parser<char> for Next {
  fn parse(&self, input: String) -> ParserResult<char> {
    let mut chars = input.chars();
    if let Some(char) = chars.next() {
      Ok(ResultData::new(char, chars.collect()))
    } else {
      Err(input)
    }
  }
}

