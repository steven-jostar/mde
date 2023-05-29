use crate::grammer::object::*;
use crate::parser::*;
use crate::tokens::*;
use crate::utils::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArraryItem {
  pub value: Value,
  pub comma: Comma,
}
impl<'a> Parser<'a> for ArraryItem {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    Ok(ArraryItem {
      value: context.parse()?,
      comma: context.parse()?,
    })
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arrary {
  start: LBracket,
  items: Vec<ArraryItem>,
  end: RBracket,
}
impl<'a> Parser<'a> for Arrary {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    Ok(Arrary {
      start: context.parse()?,
      items: repeat(context),
      end: context.parse()?,
    })
  }
}
