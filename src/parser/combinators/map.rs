use super::super::{BoxedParser, ResultData, ParserResult, Parser};

pub struct Map<'a, A, B>
{
  pub map_fn: Box<dyn Fn(A) -> B + 'a>,
  pub parser: BoxedParser<'a, A>,
}

impl<'a, A, B> Map<'a, A, B> {
  pub fn new<P, F>(parser: P, map_fn: F) -> Self
  where
    P: Parser<A> + 'a,
    F: Fn(A) -> B + 'a,
  {
    Map {
      map_fn: Box::new(map_fn),
      parser: BoxedParser::new(parser),
    }
  }
}

impl<'a, A, B> Parser<B> for Map<'a, A, B> {
  fn parse(&self, input: String) -> ParserResult<B> {
    self.parser.parse(input).map(|result| {
      ResultData::new((self.map_fn)(result.output), result.next_input)
    })
  }
}

pub fn map<'a, P, A, B, F>(parser: P, map_fn: F) -> Map<'a, A, B>
where
  P: Parser<A> + 'a,
  F: Fn(A) -> B + 'a
{
  Map::new(parser, map_fn)
}
