use super::interpret::Interpret;
use super::common::ParseError;

pub trait BinaryProvider<'a> {
  fn read_as<T: Interpret + Sized> (&mut self)
    -> Result<T, ParseError>;

  fn peek (&self, offset: usize) -> Result<u8, ParseError>;
  fn read (&mut self, length: usize) -> Result<&'a [u8], ParseError>;
  fn read_str (&mut self, length: usize) -> Result<String, ParseError>;

  // Seeking doesn't perform reads of the data,
  // so we don't need to worry about OutOfBounds
  fn seek (&mut self, offset: usize);
  fn skip (&mut self, offset: usize);
  fn back (&mut self, offset: usize);

  fn is_eof (&self) -> bool;
}
