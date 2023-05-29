use crate::parser::*;
use crate::tokens::*;

pub fn delimited<'a, D: Parser<'a> + Token>(
  context: &mut ParserContext<'a>,
) -> Result<String, String> {
  if context.parse::<D>().is_ok() {
    let mut content = String::new();
    loop {
      let mut peeker = context.peekable();
      if let Some(char) = peeker.peek() {
        if char == &D::text() {
          context.next().unwrap();

          return Ok(content);
        } else {
          content.push(context.next().unwrap())
        }
      } else {
        return Err("Failed to parse delimited content: null".to_string());
      }
    }
  } else {
    Err("Failed to parse delimited content: no first delimiter".to_string())
  }
}

pub fn repeat<'a, T: Parser<'a>>(context: &mut ParserContext<'a>) -> Vec<T> {
  let mut results = vec![];
  loop {
    if let Ok(result) = context.parse::<T>() {
      results.push(result)
    } else {
      return results;
    }
  }
}

pub fn skip_white_spaces(context: &mut ParserContext<'_>) {
  loop {
    let mut peeker = context.peekable();
    if let Some(char) = peeker.peek() {
      if " \t\n".contains(*char) {
        context.next().unwrap();
      } else {
        break;
      }
    } else {
      break;
    }
  }
}
