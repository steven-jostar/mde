// To build the mapping between raw char and token types, we must maintain a hashmap at runtime
// But sometimes we want more flexible because the hashmap will limit the implementation way to do that.
// So we can't use HashMap as the default implementation, we want more free.

pub trait TokenType {}
pub type BoxedTokenType = Box<dyn TokenType>;

/// All basic type depend on this trait, it's the base of the mde.
pub trait Token {
  /// Get raw content of the token.
  fn raw(&self) -> char;
  /// Get type of the token.
  fn typ(&self) -> BoxedTokenType;
}
