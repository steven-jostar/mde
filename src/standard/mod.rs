pub struct Header {
  pub level: u8,
}

pub trait Parser {
  type Target;
  fn parse(&self, input: &'static str) -> Option<Self::Target>;
}

impl<T: Fn(&'static str) -> Option<&'static str>> Parser for T {
  type Target = &'static str;

  fn parse(&self, input: &'static str) -> Option<Self::Target> {
    self(input)
  }
}

pub fn literal(target: &'static str) -> impl Fn(&'static str) -> Option<&'static str> {
  move |input: &'static str| {
    match input.get(0..target.len()) {
      Some(next) if next == target => Some(next),
      _ => None,
    }
  }
}
