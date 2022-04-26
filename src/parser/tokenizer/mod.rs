use token::RawToken;
use token_table::RawTokenTable;

pub mod token;
pub mod token_table;


pub struct Tokenizer<T, Y>
where
  T: Iterator<Item = char>,
  Y: RawTokenTable,
{
  pub chars_stream: T,
  pub token_table: Y,
}

impl<U, V> Tokenizer<U, V>
where
  U: Iterator<Item = char>,
  V: RawTokenTable
{
  pub fn new(
    chars: U,
    token_table: V
  ) -> Self {
    Tokenizer {
      chars_stream: chars,
      token_table
    }
  }
}

impl<T, U> Iterator for Tokenizer<T, U>
where
  T: Iterator<Item = char>,
  U: RawTokenTable
{
  type Item = RawToken;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(char) = self.chars_stream.next() {
      Some(self.token_table.build_token(&char).unwrap())
    } else {
      None
    }
  }
}

#[test]
fn dick() {
    use std::collections::HashMap;
    use self::token::RawTokenType;

    struct TokenTableImpl;
    impl RawTokenTable for TokenTableImpl {
        fn get_tokens_table(&self) -> HashMap<char, RawTokenType> {
            HashMap::from([
                ('a', RawTokenType::Char),
                ('b', RawTokenType::Char),
                ('c', RawTokenType::Char),
                ('d', RawTokenType::Symbol),
                ('e', RawTokenType::Symbol),
                ('f', RawTokenType::Symbol),
            ])
        }
    }
    let mut tokenizer = Tokenizer::new(
        vec!['a', 'b', 'c', 'd', 'e', 'f'].into_iter(),
        TokenTableImpl,
    );
    assert_eq!(tokenizer.next().unwrap(), RawToken::new('a', RawTokenType::Char));
    tokenizer.next();
    tokenizer.next();
    assert_eq!(tokenizer.next().unwrap(), RawToken::new('d', RawTokenType::Symbol));
}
