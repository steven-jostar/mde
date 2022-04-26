/// The types of tokens
/// They used to divide symbol and char to simplify resolving of token and lex convertion.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    /// i.e. symbols what need me to care about
    Symbol,
    /// i.e. normal chars
    Char,
}

#[derive(Debug, PartialEq)]
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