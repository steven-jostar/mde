use crate::parser::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Boolean {
  True,
  False,
}
impl<'a> Parser<'a> for Boolean {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    if context.peekable().take(4).collect::<String>() == *"true" {
      let _ = context.advance_by(4);
      Ok(Self::True)
    } else if context.peekable().take(5).collect::<String>() == *"false" {
      let _ = context.advance_by(5);
      Ok(Self::False)
    } else {
      Err("Failed".to_string())
    }
  }
}
