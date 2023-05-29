use crate::grammer::arrary::*;
use crate::grammer::object::*;
use crate::parser::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Document {
  Object(Object),
  Arrary(Arrary),
}
impl<'a> Parser<'a> for Document {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    context
      .parse::<Object>()
      .map(Self::Object)
      .or(context.parse::<Arrary>().map(Self::Arrary))
  }
}
