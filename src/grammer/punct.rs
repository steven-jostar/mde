use crate::parser::*;
use crate::tokens::*;
use crate::utils::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Punct<T, P> {
  Con(T, P),
  End(T),
}
impl<'a, T, P> Parser<'a> for Punct<T, P>
where
  T: Parser<'a>,
  P: Parser<'a> + Token,
{
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    if let Ok(t) = context.parse::<T>() {
      if let Ok(p) = context.parse::<P>() {
        Ok(Self::Con(t, p))
      } else {
        Ok(Self::End(t))
      }
    } else {
      Err("failed".to_string())
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Punctuated<T, P>(pub Vec<Punct<T, P>>);
impl<'a, T, P> Parser<'a> for Punctuated<T, P>
where
  T: Parser<'a> + std::fmt::Debug,
  P: Parser<'a> + Token + std::fmt::Debug,
{
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized,
  {
    let vec = repeat::<Punct<T, P>>(context);
    let iter = dbg!(vec.iter());
    if iter.rev().skip(1).all(|t| matches!(t, Punct::Con(_, _))) {
      Ok(Punctuated(vec))
    } else {
      Err("Failed to parse".to_string())
    }
  }
}
