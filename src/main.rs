use mdel::grammer::document::*;
use mdel::parser::*;
use std::fs::read_to_string;
use std::path::*;

fn main() {
  let file_path = Path::new("/home/sx/Codes/rust/mdel/test.json");
  let raw = read_to_string(file_path).unwrap();
  let mut context = ParserContext::new(&raw);

  dbg!(context.parse::<Document>().unwrap());
}
