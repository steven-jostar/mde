use crate::parser::*;
use crate::tokens::*;

pub fn is_number(char: char) -> bool {
  "1234567890".contains(char)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnsignedNum(pub usize);
impl<'a> Parser<'a> for UnsignedNum {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    let mut number = String::new();
    if let Some(char) = context.peekable().peek() {
      if is_number(*char) {
        context.next().unwrap();
        number.push(*char);
        loop {
          let mut peeker = context.peekable();
          if let Some(char) = peeker.peek() {
            if is_number(*char) {
              number.push(context.next().unwrap());
            } else {
              return Ok(UnsignedNum(number.parse().unwrap()));
            }
          } else {
            return Ok(UnsignedNum(number.parse().unwrap()));
          }
        }
      } else {
        return Err("Failed to parse".to_string());
      }
    } else {
      return Err("Error: Empty string".to_string());
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NumberSym {
  Positive(Plus),
  Negative(Minus),
}
impl<'a> Parser<'a> for NumberSym {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    context
      .parse::<Plus>()
      .map(NumberSym::Positive)
      .or(context.parse::<Minus>().map(NumberSym::Negative))
      .or(Ok(NumberSym::Positive(Plus)))
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LitNumber {
  pub sym: NumberSym,
  pub integer: UnsignedNum,
  pub point: Point,
  pub frac: Option<UnsignedNum>,
}
impl<'a> Parser<'a> for LitNumber {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    Ok(LitNumber {
      sym: context.parse()?,
      integer: context.parse()?,
      point: context.parse().unwrap_or(Point),
      frac: context.parse().ok(),
    })
  }
}
#[test]
fn test_num() {
  let test = "-114.514";
  let mut context = ParserContext::new(test);

  assert_eq!(
    context.parse::<LitNumber>(),
    Ok(LitNumber {
      sym: NumberSym::Negative(Minus),
      integer: UnsignedNum(114),
      point: Point,
      frac: Some(UnsignedNum(514)),
    })
  );
}
