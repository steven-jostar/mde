use crate::parser::*;
use crate::utils::*;

pub trait Token {
  fn text() -> char;
}

macro_rules! def_token {
  { $(($name:ident, $content:expr)),* } => {
    $(
      #[derive(Clone, Debug, Copy, PartialEq, Eq)]
      pub struct $name;
      impl<'a> Parser<'a> for $name {
        fn parse(context: &mut ParserContext<'a>) -> Result<Self, String> {
          skip_white_spaces(context);
          if let Some(char) = context.peekable().peek() {
            if char == &$content {
              context.next().unwrap();

              return Ok($name);
            }
          }
          return Err("Failed to parse $name".to_string());
        }
      }
      impl Token for $name {
        fn text() -> char {
          $content
        }
      }
    )*
  }
}

def_token! {
  (Comma, ','),
  (Semicolon, ';'),
  (LBrace, '{'),
  (RBrace, '}'),
  (LBracket, '['),
  (RBracket, ']'),
  (LParen, '('),
  (RParen, ')'),
  (SQuote, '\''),
  (DQuote, '\"'),
  (Point, '.'),
  (Plus, '+'),
  (Minus, '-'),
  (Colon, ':')
}

// This is OK now
#[test]
fn test_tokens() {
  let test_1 = ",This is a test line";
  let test_2 = ".This is a test line";
  let mut context_1 = ParserContext::new(test_1);
  let mut context_2 = ParserContext::new(test_2);

  assert_eq!(context_1.parse(), Ok(Comma));
  assert_eq!(
    context_1.clone().collect::<String>(),
    "This is a test line".to_string()
  );

  // An failed parsing should not consume `ParserContext`.
  assert_eq!(
    context_2.parse::<Comma>(),
    Err("Failed to parse $name".to_string())
  );
  assert_eq!(
    context_2.clone().collect::<String>(),
    ".This is a test line".to_string()
  );
}
