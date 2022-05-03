pub type ParserResult<O> = Result<(String, O), String>;

pub trait Parser<'a, Output: 'a> {
  fn parse(&self, input: String) -> ParserResult<Output>;

  fn map<A, F>(self, map_fn: F) -> BoxedParser<'a, A>
  where
    F: Fn(Output) -> A + 'a,
    Self: Sized + 'a
  {
    BoxedParser::new(
      move |input: String| {
        self
          .parse(input)
          .map(|(next_input, result)| (next_input, map_fn(result)))
      }
    )
  }
  fn pair<P, A>(self, parser: P) -> BoxedParser<'a, (Output, A)>
  where
    P: Parser<'a, A> + 'a,
    A: 'a,
    Self: Sized + 'a
  {
    BoxedParser::new(
      move |input: String| {
        self.parse(input).and_then(|(next_input, result_a)| {
          parser.parse(next_input).map(|(next_input, result_b)| {
            (next_input, (result_a, result_b))
          })
        })
      }
    )
  }
  fn left<P, A>(self, parser: P) -> BoxedParser<'a, Output>
  where
    P: Parser<'a, A> + 'a,
    A: 'a,
    Self: Sized + 'a
  {
    self.pair(parser).map(|(left, _right)| left)
  }
  fn right<P, A>(self, parser: P) -> BoxedParser<'a, A>
  where
    P: Parser<'a, A> + 'a,
    A: 'a,
    Self: Sized + 'a
  {
    self.pair(parser).map(|(_left, right)| right)
  }
  fn until<F>(&'a self, condition: F) -> BoxedParser<'a, Vec<Output>>
  where
    F: Fn(Output) -> Option<Output> + 'a,
    Self: Sized + 'a,
  {
    BoxedParser::new(
      move |input: String| {
        let mut result = Vec::new();
        let mut input = input;
        if let Ok((first_input, first_result)) = self.parse(input.clone()) {
          if let Some(first_result) = condition(first_result) {
            result.push(first_result);
            input = first_input;
          } else {
            return Err(first_input);
          }
        } else {
          return Err(input);
        }
        while let Ok((next_input, item)) = self.parse(input.clone()) {
          if let Some(item) = condition(item) {
            result.push(item);
            input = next_input;
            continue;
          } else {
            return Ok((next_input, result))
          }
        }
        Err(input)
      }
    )
  }

  fn condition<F>(self, condition: F) -> BoxedParser<'a, Output>
  where
    F: Fn(Output) -> Option<Output> + 'a,
    Self: Sized + 'a,
  {
    BoxedParser::new(
      move |input: String| {
        self.parse(input).and_then(|(next_input, result)| {
          match condition(result) {
            Some(result) => Ok((next_input, result)),
            None => Err(next_input),
          }
        })
      }
    )
  }
}

impl<'a, F, Output: 'a> Parser<'a, Output> for F
where
  F: Fn(String) -> ParserResult<Output>
{
  fn parse(&self, input: String) -> ParserResult<Output> {
    self(input)
  }
}

pub struct BoxedParser<'a, O> {
  parser: Box<dyn Parser<'a, O> + 'a>,
}

impl<'a, O> BoxedParser<'a, O> {
  pub fn new<P>(parser: P) -> Self
  where P: Parser<'a, O> + 'a
  {
    BoxedParser {
      parser: Box::new(parser),
    }
  }
}

impl<'a, O> Parser<'a, O> for BoxedParser<'a, O> {
  fn parse(&self, input: String) -> ParserResult<O> {
    self.parser.parse(input)
  }
}
