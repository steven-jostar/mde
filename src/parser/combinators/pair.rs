use super::super::{BoxedParser, ResultData, ParserResult, Parser};

pub struct Pair<'a, A, B> {
  first: BoxedParser<'a, A>,
  second: BoxedParser<'a, B>,
}

impl<'a, A, B> Parser<(A, B)> for Pair<'a, A, B> {
  fn parse(&self, input: String) -> ParserResult<(A, B)> {
    self.first.parse(input).and_then(|result_a| {
      self.second.parse(result_a.next_input).map(|result_b| {
        ResultData::new((result_a.output, result_b.output), result_b.next_input)
      })
    })
  }
}

impl<'a, A, B> Pair<'a, A, B> {
  pub fn new<F, S>(first: F, second: S) -> Self
  where
    F: Parser<A> + 'a,
    S: Parser<B> + 'a,
  {
    Pair {
      first: BoxedParser::new(first),
      second: BoxedParser::new(second),
    }
  }
}

pub fn pair<'a, F, S, A, B>(first: F, second: S) -> Pair<'a, A, B>
where
  F: Parser<A> + 'a,
  S: Parser<B> + 'a,
{
  Pair::new(first, second)
}
