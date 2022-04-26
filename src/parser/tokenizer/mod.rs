use token::Token;
use token_table::TokenTable;

pub mod token;
pub mod token_table;


pub struct Tokenizer<T, Y>
where
    T: Iterator<Item = char>,
    Y: TokenTable,
{
    pub chars_stream: T,
    pub token_table: Y,
}

impl<U, V> Tokenizer<U, V>
where
    U: Iterator<Item = char>,
    V: TokenTable
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
    U: TokenTable
{
    type Item = Token;

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
    use self::token::TokenType;

    struct TokenTableImpl;
    impl TokenTable for TokenTableImpl {
        fn get_tokens_table(&self) -> HashMap<char, TokenType> {
            HashMap::from([
                ('a', TokenType::Char),
                ('b', TokenType::Char),
                ('c', TokenType::Char),
                ('d', TokenType::Symbol),
                ('e', TokenType::Symbol),
                ('f', TokenType::Symbol),
            ])
        }
    }
    let mut tokenizer = Tokenizer::new(
        vec!['a', 'b', 'c', 'd', 'e', 'f'].into_iter(),
        TokenTableImpl,
    );
    assert_eq!(tokenizer.next().unwrap(), Token::new('a', TokenType::Char));
    tokenizer.next();
    tokenizer.next();
    assert_eq!(tokenizer.next().unwrap(), Token::new('d', TokenType::Symbol));
}