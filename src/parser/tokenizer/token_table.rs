use std::collections::HashMap;
use super::token::{RawToken, RawTokenType};

/// Saves all the tokens and defined some useful functions used to operate the tokens and convert types between char and token.
/// Namely, it provides a easy way to build the mapping between char and token.
/// Ideally, it maintained a HashMap type what saved the information about char and token.
/// When `get()` method called, it will retrieve corresponding token type in the `HashMap` type by given char.
pub trait RawTokenTable {
    fn get_tokens_table(&self) -> HashMap<char, RawTokenType>;
    fn get_typ(&mut self, value: &char) -> Option<RawTokenType> {
        let tokens_table = self.get_tokens_table();
        let maybe_token_type = tokens_table.get(value);
        if let Some(token_type) = maybe_token_type {
            Some(*token_type)
        } else {
            None
        }
    }
    fn build_token(&mut self, value: &char) -> Option<RawToken> {
        if let Some(typ) = self.get_typ(value) {
            Some(RawToken::new(*value, typ))
        } else {
            None
        }
    }
}
