use super::parser::{
  Parser,
  ParserResult,
  BoxedParser,
  and,
  left,
  right,
};

// Basic Unit - Cannot be split into smaller units (atomic structure).
// Composed Structure - Consist of Basic unit

pub trait Unit {
  fn content(&self) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Header {
  pub level: usize,
  pub content: String,
}

pub fn next(input: String) -> ParserResult<char> {
  let mut chars = input.chars();
  match chars.next() {
    Some(next) => Ok((chars.collect(), next)),
    None => Err(input),
  }
}

pub fn pair_with<'a>(pair: String) -> BoxedParser<'a, String> {
  BoxedParser::new(
    move |input: String| {
      let mut chars = input.chars();
      let mut results: Vec<char> = Vec::new();
      loop {
        let current_stream: String = chars.clone().collect();
        if current_stream.starts_with(&pair) {
          let mut chars = chars.skip(pair.len());
          loop {
            if let Some(char) = chars.next() {
              results.push(char);
            } else {
              return Err(chars.collect());
            }
            let current_stream: String = chars.clone().collect();
            if current_stream.starts_with(&pair) {
              return Ok((chars.skip(pair.len()).collect(), results.into_iter().collect()));
            }
          }
        }
        if chars.next().is_none() {
          return Err("".to_string());
        }
      }
    }
  )
}

pub fn header<'a>() -> BoxedParser<'a, Header> {
  and(
    next
      .until_one(|(_, char)| *char == '#')
      .map(|chars| chars.len()),
    next
      .until_one(|(_, char)| (*char != '\n'))
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
      next.until_one(|(_, char)| !(*char == '*' || *char == '_')),
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
      .until_one(|(_, char)| *char != '*')
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
        .until_one(|(_, char)| *char != '\n')
        .map(|chars| {
          Reference(chars.into_iter().collect())
        })
    ),
  )
}

#[derive(Debug, PartialEq, Eq)]
pub struct InlineCodeBlock(String);

pub fn inline_codeblock<'a>() -> BoxedParser<'a, InlineCodeBlock> {
  right(
    next
      .condition(|(_, char)| *char == '`'),
    left(
      next
        .until(|(_, char)| *char != '`')
        .map(|chars| {
          InlineCodeBlock(chars.into_iter().collect::<String>())
        }),
      next.condition(|(_, char)| *char == '`')
    )
  )
}

#[test]
fn test() {
  let pairing_with_suit = "****awdliajwdlij****".to_string();
  let header_suit = "## Header\n";
  let bold_suit = "**Bold**";
  let italic_suit = "*Italic*";
  let reference_suit = "> Reference\n";
  let inline_codeblock_suit = "`Inline codeblock`";
  assert_eq!(
    Ok((
      "".to_string(),
      "awdliajwdlij".to_string()
    )),
    pair_with("****".to_string()).parse(pairing_with_suit)
  );
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
  assert_eq!(
    Ok((
      "".to_string(),
      InlineCodeBlock("Inline codeblock".to_string())
    )),
    inline_codeblock().parse(inline_codeblock_suit.into())
  );
}
