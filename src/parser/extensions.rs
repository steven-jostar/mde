use super::{
  Parser,
  ResultData,
  Map,
  Until,
  Condition,
};

pub trait ParserExt<O>: Parser<O> {
  fn map<'a, M, F>(self, map_fn: F) -> Map<'a, O, M>
  where
    F: Fn(O) -> M + 'a,
    Self: Sized + 'a,
  {
    Map::new(self, map_fn)
  }
  fn until<'a, F>(self, condition: F) -> Until<'a, O, F>
  where
    F: for<'b> Fn(&'b ResultData<O>) -> bool + 'a,
    Self: Sized + 'a,
  {
    Until::new(condition, self)
  }
  fn condition<'a, F>(self, condition: F) -> Condition<'a, O, F>
  where
    F: for<'b> Fn(&'b ResultData<O>) -> bool + 'a,
    Self: Sized + 'a,
  {
    Condition::new(self, condition)
  }
}

impl<P, O> ParserExt<O> for P
where
  P: Parser<O>
{}
