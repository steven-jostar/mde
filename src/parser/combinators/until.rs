use crate::parser::{
  Parser,
  ParserResult,
  ResultData,
  BoxedParser,
};

pub struct Until<'a, A, F>
where
  F: for<'b> Fn(&'a ResultData<A>) -> bool + 'a,
  A: 'a,
{
  parser: BoxedParser<'a, A>,
  condition: Box<F>,
}

impl<'a, A, F> Parser<Vec<A>> for Until<'a, A, F>
where
  F: for<'b> Fn(&'b ResultData<A>) -> bool + 'a,
  A: 'a,
{
  fn parse(&self, input: String) -> ParserResult<Vec<A>> {
    let mut results: Vec<A> = Vec::new();
    let mut next_input: String = input;
    loop {
      if let Ok(result) = self.parser.parse(next_input.clone()) {
        if (self.condition)(&result) {
          results.push(result.output);
          next_input = result.next_input;
        } else {
          break Ok(ResultData::new(results, result.next_input));
        }
      } else {
        return Err(next_input);
      }
    }
  }
}

impl<'a, A, F> Until<'a, A, F>
where
  A: 'a,
  F: for<'b> Fn(&'b ResultData<A>) -> bool + 'a,
{
  pub fn new<P>(condition: F, parser: P) -> Self
  where
    P: Parser<A> + 'a,
  {
    Until {
      condition: Box::new(condition),
      parser: BoxedParser::new(parser),
    }
  }
}

pub fn until<'a, A, P, F>(condition: F, parser: P) -> Until<'a, A, F>
where
  P: Parser<A> + 'a,
  F: for<'b> Fn(&'b ResultData<A>) -> bool + 'a,
{
  Until::new(condition, parser)
}
