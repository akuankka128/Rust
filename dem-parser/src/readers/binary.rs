use crate::interpret::Interpret;
use crate::provider::BinaryProvider;
use crate::common::ParseError;

pub struct BinaryReader<'a> {
  pub(self) read_index: usize,
  pub(self) binary_src: &'a [u8],
}

impl<'a> BinaryReader<'a> {
  pub fn from_str<'b> (src: &'b str) -> BinaryReader<'b> {
    BinaryReader {
      binary_src: src.as_bytes(),
      read_index: 0,
    }
  }

  pub fn from_bytes<'b: 'a> (src: &'b [u8]) -> BinaryReader<'a> {
    Self {
      binary_src: src,
      read_index: 0,
    }
  }
}

impl<'a> BinaryProvider<'a> for BinaryReader<'a> {
  fn peek (&self, offset: usize) -> Result<u8, ParseError> {
    let ptr = self.read_index + offset;
    let deref = self.binary_src.get(ptr);
    if let Some(value) = deref {
      Ok(*value)
    }
    else {
      let max = self.binary_src.len();
      Err(ParseError::OutOfBounds(ptr, max))
    }
  }

  fn read (&mut self, length: usize) -> Result<&'a [u8], ParseError> {
    let start = self.read_index;
    let end = start + length;

    if end > self.binary_src.len() {
      let max = self.binary_src.len();
      Err(ParseError::OutOfBounds(end, max))
    }
    else {
      self.read_index += length;
      Ok(&self.binary_src[start..end])
    }
  }

  fn read_as<T: Interpret + Sized> (&mut self)
    -> Result<T, ParseError>
  {
    let bytes = std::mem::size_of::<T>();
    let data = self.read(bytes)?;
    Ok(T::from_bytes(data))
  }

  fn read_str (&mut self, length: usize)
    -> Result<String, ParseError>
  {
    let data = self.read(length)?;
    Ok(String::from_bytes(data))
  }

  fn seek (&mut self, offset: usize) {
    self.read_index = offset;
  }

  fn skip (&mut self, offset: usize) {
    self.read_index += offset;
  }

  fn back (&mut self, offset: usize) {
    self.read_index -= offset;
  }

  fn is_eof (&self) -> bool {
    self.read_index >= self.binary_src.len()
  }
}
