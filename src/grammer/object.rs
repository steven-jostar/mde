use crate::grammer::arrary::*;
use crate::grammer::boolean::*;
use crate::grammer::number::*;
use crate::grammer::punct::*;
use crate::grammer::string::*;

use crate::parser::*;
use crate::tokens::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Key(pub LitString);
impl<'a> Parser<'a> for Key {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    Ok(Key(context.parse()?))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
  Number(LitNumber),
  String(LitString),
  Object(Object),
  Arrary(Arrary),
  Boolean(Boolean),
}
impl<'a> Parser<'a> for Value {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    context
      .parse::<LitNumber>()
      .map(Value::Number)
      .or(context.parse::<LitString>().map(Value::String))
      .or(context.parse::<Object>().map(Value::Object))
      .or(context.parse::<Arrary>().map(Value::Arrary))
      .or(context.parse::<Boolean>().map(Value::Boolean))
  }
}

/// ```
/// use mdel::tokens::*;
/// use mdel::grammer::number::*;
/// use mdel::grammer::string::*;
/// use mdel::grammer::object::*;
/// use mdel::parser::*;
///
/// let test = "\"name\": 114514,";
/// let mut context = ParserContext::new(test);
/// assert_eq!(context.parse::<ObjectItem>(), Ok(ObjectItem {
///   key: Key(LitString {
///     quote_1: DQuote,
///     content: "name".to_string(),
///     quote_2: DQuote,
///   }),
///   colon: Colon,
///   value: Value::Number(LitNumber {
///     sym: NumberSym::Positive(Plus),
///     integer: UnsignedNum(114514),
///     point: Point,
///     frac: None,
///   }),
/// }))
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObjectItem {
  pub key: Key,
  pub colon: Colon,
  pub value: Value,
}
impl<'a> Parser<'a> for ObjectItem {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    Ok(ObjectItem {
      key: context.parse()?,
      colon: context.parse()?,
      value: context.parse()?,
    })
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Object {
  pub start: LBrace,
  pub items: Punctuated<ObjectItem, Comma>,
  pub end: RBrace,
}
impl<'a> Parser<'a> for Object {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    Ok(Object {
      start: context.parse()?,
      items: context.parse()?,
      end: context.parse()?,
    })
  }
}
