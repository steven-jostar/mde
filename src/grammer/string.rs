use crate::parser::*;
use crate::tokens::*;
use crate::utils::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LitString {
  pub quote_1: DQuote,
  pub content: String,
  pub quote_2: DQuote,
}
impl<'a> Parser<'a> for LitString {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    Ok(LitString {
      quote_1: DQuote,
      content: delimited::<DQuote>(context)?,
      quote_2: DQuote,
    })
  }
}

#[test]
fn test_string() {
  let test_1 = "\"This is a test\"";
  let test_2 = "This is a test";
  let mut context_1 = ParserContext::new(test_1);
  let mut context_2 = ParserContext::new(test_2);

  assert_eq!(
    context_1.parse::<LitString>(),
    Ok(LitString {
      quote_1: DQuote,
      content: "This is a test".to_string(),
      quote_2: DQuote
    })
  );
  assert_eq!(context_1.collect::<String>(), "".to_string());

  let _ = context_2.parse::<LitString>();
  assert_eq!(context_2.collect::<String>(), "This is a test".to_string());
}
