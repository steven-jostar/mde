use crate::parser::{
  Parser,
  ParserResult,
  ResultData,
  BoxedParser,
};

pub struct Condition<'a, A, F>
where
  A: 'a,
  F: for<'b> Fn(&'b ResultData<A>) -> bool + 'a,
{
  parser: BoxedParser<'a, A>,
  condition: Box<F>,
}

impl<'a, A, F> Parser<A> for Condition<'a, A, F>
where
  A: 'a,
  F: for<'b> Fn(&'b ResultData<A>) -> bool + 'a,
{
  fn parse(&self, input: String) -> ParserResult<A> {
    if let Ok(result) = self.parser.parse(input.clone()) {
      if (self.condition)(&result) {
        Ok(result)
      } else {
        Err(input)
      }
    } else {
      Err(input)
    }
  }
}

impl<'a, A, F> Condition<'a, A, F>
where
  A: 'a,
  F: for<'b> Fn(&'b ResultData<A>) -> bool + 'a,
{
  pub fn new<P>(parser: P, condition: F) -> Self
  where
    P: Parser<A> + 'a,
  {
    Condition {
      parser: BoxedParser::new(parser),
      condition: Box::new(condition),
    }
  }
}

pub fn condition<'a, A, F, P>(parser: P, condition: F) -> Condition<'a, A, F>
where
  A: 'a,
  F: for<'b> Fn(&'b ResultData<A>) -> bool + 'a,
  P: Parser<A> + 'a,
{
  Condition::new(parser, condition)
}
