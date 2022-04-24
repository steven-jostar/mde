use std::collections::HashMap;

/// The types of tokens
/// They used to divide symbol and char to simplify resolving of token and lex convertion.
#[derive(Clone, Copy)]
pub enum TokenType {
    /// i.e. symbols what need me to care about
    Symbol,
    /// i.e. normal chars
    Char,
}

pub struct Token {
    pub value: char,
    pub typ: TokenType,
}

impl Token {
    pub fn new(value: char, typ: TokenType) -> Self {
        Token {
            value,
            typ,
        }
    }
}

/// Saves all the tokens and defined some useful functions used to operate the tokens and convert types between char and token.
/// Namely, it provides a easy way to build the mapping between char and token.
/// Ideally, it maintained a HashMap type what saved the information about char and token.
/// When `get()` method called, it will retrieve corresponding token type in the `HashMap` type by given char.
pub trait TokenTable {
    fn get_tokens_table(&self) -> HashMap<char, TokenType>;
    fn get_typ(&mut self, value: &char) -> Option<TokenType> {
        let tokens_table = self.get_tokens_table();
        let maybe_token_type = tokens_table.get(value);
        if let Some(token_type) = maybe_token_type {
            Some(*token_type)
        } else {
            None
        }
    }
    fn build_token(&mut self, value: &char) -> Option<Token> {
        if let Some(typ) = self.get_typ(value) {
            Some(Token::new(*value, typ))
        } else {
            None
        }
    }
}

pub struct Tokenizer<T, Y>
where
    T: Iterator<Item = char>,
    Y: TokenTable,
{
    pub chars_stream: T,
    pub token_table: Y,
}

impl<U, V> Tokenizer<U, V> {
    pub fn new<T>(
        chars: Box<T>,
        token_table: Box<dyn TokenTable>
    ) -> Self
    where T: Iterator<Item = char> + 'static {
        Tokenizer {
            chars_stream: chars,
            token_table
        }
    }
}

impl Iterator for Tokenizer {
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
    let tokenizer = Tokenizer::new(
        Box::new(vec!['a', 'b', 'c', 'd', 'e', 'f'].iter()),
        Box::new(TokenTableImpl)
    );
}