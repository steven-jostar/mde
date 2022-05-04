use super::parser::{
  Parser,
  ParserResult,
  BoxedParser,
  and,
  left,
  right,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
  pub level: usize,
  pub content: String,
}

pub fn next(input: String) -> ParserResult<char> {
  match input.chars().next() {
    Some(next) => Ok((input[next.len_utf8()..].to_string(), next)),
    None => Err(input),
  }
}

pub fn start_with<'a>(str: String) -> BoxedParser<'a, String> {
  BoxedParser::new(
    move |input: String| {
      let mut input = input;
      if input.starts_with(&str) {
        Ok((input.drain(..str.len()).collect(), str.clone()))
      } else {
        Err(input)
      }
    }
  )
}

pub fn header<'a>() -> BoxedParser<'a, Header> {
  and(
    next
      .until(|(_, char)| *char == ' ')
      .map(|chars| chars.len()),
    next
      .until(|(_, char)| (*char == '\n'))
      .map(|chars| chars.into_iter().collect::<String>())
  ).map(|(level, content)| {
    Header {
      level,
      content,
    }
  })
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bold(String);
#[derive(Debug, PartialEq, Eq)]
pub struct Italic(String);

pub fn bold<'a>() -> BoxedParser<'a, Bold> {
  right(
    next
      .condition(|(_, char)| *char == '*' || *char == '_')
      .repeat(2),
    left(
      next.until(|(_, char)| !(*char == '*' || *char == '_')),
      next.condition(|(_, char)| *char == '*' || *char == '_'),
    )
      .map(|chars| {
        Bold(chars.into_iter().collect::<String>())
      })
  )
}

pub fn italic<'a>() -> BoxedParser<'a, Italic> {
  right(
    next.condition(|(_, char)| *char == '*'),
    next
      .until(|(_, char)| !(*char == '*'))
      .map(|chars| {
        Italic(chars.into_iter().collect::<String>())
      })
  )
}

#[derive(Debug, PartialEq, Eq)]
pub struct Reference(String);

pub fn reference<'a>() -> BoxedParser<'a, Reference> {
  right(
    next
      .condition(|(_, char)| *char == '>'),
    right(
      next
        .condition(|(_, char)| *char == ' '),
      next
        .until(|(_, char)| !(*char == '\n'))
        .map(|chars| {
          Reference(chars.into_iter().collect())
        })
    ),
  )
}

#[test]
fn test() {
  let header_suit = "## Header\n";
  let bold_suit = "**Bold**";
  let italic_suit = "*Italic*";
  let reference_suit = "> Reference\n";
  assert_eq!(
    Ok((
      "".to_string(),
      Header {
        level: 2,
        content: "Header".into(),
      })
    ),
    header().parse(header_suit.into())
  );
  assert_eq!(
    Ok((
      "".to_string(),
      Bold("Bold".to_string())
    )),
    bold().parse(bold_suit.into())
  );
  assert_eq!(
    Ok((
      "".to_string(),
      Italic("Italic".to_string())
    )),
    italic().parse(italic_suit.into())
  );
  assert_eq!(
    Ok((
      "".to_string(),
      Reference("Reference".into())
    )),
    reference().parse(reference_suit.into())
  );
}
