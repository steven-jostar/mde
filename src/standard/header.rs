use crate::parser::{
  Parser,
  ParserResult,
  ParserExt,
  pair,
};
use super::utils::Next;

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
  pub content: String,
  pub level: u8,
}

pub struct HeaderParser
{
  parser: Box<dyn Parser<Header>>
}

impl Parser<Header> for HeaderParser {
  fn parse(&self, input: String) -> ParserResult<Header> {
    self.parser.parse(input)
  }
}

impl Default for HeaderParser {
  fn default() -> Self {
    HeaderParser {
      parser: Box::new(
        pair(
          Next
            .condition(|result| result.output == '#' || result.output == ' ')
            .until(|result| result.output != ' ')
            .map(|chars| chars.len() as u8)
            .condition(|result| result.output <= 6),
          Next
            .until(|result| result.output != '\n')
            .map(|chars| chars.into_iter().collect::<String>())
        )
        .map(|(level, content)| {
          Header {
            level,
            content,
          }
        })
      )
    }
  }
}

#[test]
fn test_header() {
  use crate::parser::ResultData;
  let header_parser = HeaderParser::default();
  let test_suit = "###### Header\n".to_string();
  assert_eq!(
    Ok(ResultData::new(
      Header {
        level: 6,
        content: "Header".to_string(),
      },
      "".to_string()
    )),
    header_parser.parse(test_suit)
  )
}
