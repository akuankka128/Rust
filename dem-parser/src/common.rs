#[derive(Debug)]
pub enum ParseError {
  OutOfBounds(usize, usize),
  InvalidFormat(String),
}

impl ToString for ParseError {
  fn to_string (&self) -> String {
    match self {
      ParseError::OutOfBounds(index, max) => {
        format!("{index} was beyond bound {max}")
      },
      ParseError::InvalidFormat(detail) => {
        format!("invalid format: {detail}")
      }
    }
  }
}
