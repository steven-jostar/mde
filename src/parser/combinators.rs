pub type ParserResult<O> = Result<(String, O), String>;

pub trait Parser<Output> {
  fn parse(&self, input: String) -> ParserResult<Output>;
}

impl<F, Output> Parser<Output> for F
where
  F: Fn(String) -> ParserResult<Output>
{
  fn parse(&self, input: String) -> ParserResult<Output> {
    self(input)
  }
}

pub fn map<P, A, B, F>(parser: P, map_fn: F) -> impl Parser<B>
where
  P: Parser<A>,
  F: Fn(A) -> B,
{
  move |input: String| {
    parser.parse(input).map(|(result, a)| (result, map_fn(a)))
  }
}

pub fn pair<P1, P2, A, B>(p1: P1, p2: P2) -> impl Parser<(A, B)>
where
  P1: Parser<A>,
  P2: Parser<B>,
{
  move |input: String| {
    p1.parse(input).and_then(|(next_result, a)| {
      p2.parse(next_result).map(|(last_result, b)| (last_result, (a, b)))
    })
  }
}

pub fn left<P1, P2, A, B>(p1: P1, p2: P2) -> impl Parser<A>
where
  P1: Parser<A>,
  P2: Parser<B>,
{
  map(pair(p1, p2), |(left, _right)| left)
}

pub fn right<P1, P2, A, B>(p1: P1, p2: P2) -> impl Parser<B>
where
  P1: Parser<A>,
  P2: Parser<B>,
{
  map(pair(p1, p2), |(_left, right)| right)
}

pub fn one_or_more<P, A>(parser: P) -> impl Parser<Vec<A>>
where
  P: Parser<(bool, A)>
{
  move |input: String| {
    let mut result: Vec<A> = Vec::new();
    let mut input = input;

    if let Ok((next_input, (_, first_item))) = parser.parse(input.clone()) {
      input = next_input;
      result.push(first_item);
    } else {
      return Err(input);
    }

    while let Ok((next_input, (can_continue, next_item))) = parser.parse(input.clone()) {
      if can_continue {
        result.push(next_item);
        input = next_input;
        continue;
      } else {
        return Ok((input, result));
      }
    }

    Ok((input, result))
  }
}
