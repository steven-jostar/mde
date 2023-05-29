use std::str::Chars;

pub struct Peekable<I: Iterator> {
  iter: I,
  peeked: Option<Option<I::Item>>,
}
impl<I: Iterator> Iterator for Peekable<I> {
  type Item = I::Item;

  fn next(&mut self) -> Option<Self::Item> {
    match self.peeked.take() {
      Some(v) => v,
      None => self.iter.next(),
    }
  }
}
impl<I: Iterator> Peekable<I> {
  pub fn peek(&mut self) -> Option<&I::Item> {
    let iter = &mut self.iter;
    self.peeked.get_or_insert_with(|| iter.next()).as_ref()
  }
}

#[derive(Debug, Clone)]
pub struct ParserContext<'a> {
  content: Chars<'a>,
  _marker: std::marker::PhantomData<&'a String>,
}
impl<'a> Iterator for ParserContext<'a> {
  type Item = char;

  fn next(&mut self) -> Option<Self::Item> {
    self.content.next()
  }
}
impl<'a> ParserContext<'a> {
  pub fn new(content: &'a str) -> Self {
    ParserContext {
      content: content.chars(),
      _marker: std::marker::PhantomData,
    }
  }
  pub fn peekable(&mut self) -> Peekable<Chars<'a>> {
    Peekable {
      iter: self.content.clone(),
      peeked: None,
    }
  }

  pub fn parse<T: Parser<'a>>(&mut self) -> Result<T, String> {
    T::parse(self)
  }
}

pub trait Parser<'a> {
  fn parse(context: &mut ParserContext<'a>) -> Result<Self, String>
  where
    Self: Sized;
}
