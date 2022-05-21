use super::super::{BoxedParser, ParserResult, Parser};

pub struct AndThen<'a, A> {
  parser: BoxedParser<'a, A>,
  condition: BoxedParser<'a, ()>,
}

impl<'a, A> Parser<A> for AndThen<'a, A> {
  fn parse(&self, input: String) -> ParserResult<A> {
    self.condition.parse(input).and_then(|result| {
      self.parser.parse(result.next_input)
    })
  }
}

impl<'a, A> AndThen<'a, A> {
  pub fn new<C, P>(condition: C, parser: P) -> Self
  where
    C: Parser<()> + 'a,
    P: Parser<A> + 'a,
  {
    AndThen {
      condition: BoxedParser::new(condition),
      parser: BoxedParser::new(parser),
    }
  }
}

pub fn and_then<'a, A, C, P>(condition: C, parser: P) -> AndThen<'a, A>
where
  P: Parser<A> + 'a,
  C: Parser<()> + 'a,
{
  AndThen::new(condition, parser)
}
