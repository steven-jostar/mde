use crate::grammer::object::*;
use crate::grammer::punct::*;
use crate::parser::*;
use crate::tokens::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArraryItem(pub Value);
impl<'a> Parser<'a> for ArraryItem {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    Ok(ArraryItem(context.parse()?))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arrary {
  start: LBracket,
  items: Punctuated<Value, Comma>,
  end: RBracket,
}
impl<'a> Parser<'a> for Arrary {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    Ok(Arrary {
      start: context.parse()?,
      items: context.parse()?,
      end: context.parse()?,
    })
  }
}
