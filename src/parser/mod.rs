pub type ParserResult<O> = Result<(String, O), String>;

pub fn map<'a, P, A, B, F>(parser: P, map_fn: F) -> BoxedParser<'a, B>
where
  P: Parser<'a, A> + 'a,
  A: 'a,
  B: 'a,
  F: Fn(A) -> B + 'a
{
  BoxedParser::new(
    move |input: String| {
      parser.parse(input).map(|(next_input, result)| (next_input, map_fn(result)))
    }
  )
}

pub fn and<'a, P1, P2, A, B>(p1: P1, p2: P2) -> BoxedParser<'a, (A, B)>
where
  A: 'a,
  B: 'a,
  P1: Parser<'a, A> + 'a,
  P2: Parser<'a, B> + 'a,
{
  BoxedParser::new(
    move |input: String| {
      p1.parse(input).and_then(|(next_input, result_a)| {
        p2.parse(next_input).map(|(next_input, result_b)| {
          (next_input, (result_a, result_b))
        })
      })
    }
  )
}

pub fn left<'a, P1, P2, A, B>(p1: P1, p2: P2) -> BoxedParser<'a, A>
where
  P1: Parser<'a, A> + 'a,
  P2: Parser<'a, B> + 'a,
  A: 'a,
  B: 'a,
{
  and(p1, p2).map(|(left, _right)| left)
}

pub fn right<'a, P1, P2, A, B>(p1: P1, p2: P2) -> BoxedParser<'a, B>
where
  P1: Parser<'a, A> + 'a,
  P2: Parser<'a, B> + 'a,
  A: 'a,
  B: 'a,
{
  and(p1, p2).map(|(_left, right)| right)
}

pub fn select<'a, P, A>(parsers: Vec<P>) -> BoxedParser<'a, A>
where
  P: Parser<'a, A> + 'a,
  A: 'a,
{
  BoxedParser::new(
    move |input: String| {
      for parser in parsers.iter() {
        if let Ok((next_input, result)) = parser.parse(input.clone()) {
          return Ok((next_input, result));
        }
      }
      Err(input)
    }
  )
}

pub trait Parser<'a, Output: 'a> {
  fn parse(&self, input: String) -> ParserResult<Output>;

  fn map<A, F>(self, map_fn: F) -> BoxedParser<'a, A>
  where
    F: Fn(Output) -> A + 'a,
    Self: Sized + 'a
  {
    map(self, map_fn)
  }

  fn until<F>(self, condition: F) -> BoxedParser<'a, Vec<Output>>
  where
    F: Fn((&'a String, &'a Output)) -> bool + 'a,
    Self: Sized + 'a,
  {
    BoxedParser::new(
      move |input: String| {
        let mut result = Vec::new();
        let mut input = input;
        if let Ok((first_input, first_result)) = self.parse(input.clone()) {
          if condition((&first_input, &first_result)) {
            result.push(first_result);
            input = first_input;
          } else {
            return Err(first_input);
          }
        } else {
          return Err(input);
        }
        while let Ok((next_input, item)) = self.parse(input.clone()) {
          if condition((&next_input, &item)) {
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

  fn repeat(self, count: usize) -> BoxedParser<'a, Vec<Output>>
  where
    Self: Sized + 'a
  {
    BoxedParser::new(
      move |input: String| {
        let mut input = input;
        let mut result = Vec::new();
        for _ in 0..count {
          if let Ok((next_input, item)) = self.parse(input.clone()) {
            input = next_input;
            result.push(item);
          } else {
            return Err(input);
          }
        }
        Ok((input, result))
      }
    )
  }

  fn condition<F>(self, condition: F) -> BoxedParser<'a, Output>
  where
    F: Fn((&'a String, &'a Output)) -> bool + 'a,
    Self: Sized + 'a,
  {
    BoxedParser::new(
      move |input: String| {
        self.parse(input).and_then(|(next_input, result)| {
          if condition((&next_input, &result)) {
            return Ok((next_input, result))
          } else {
            return Err(next_input);
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
