pub trait Interpret {
  fn from_bytes (src: &[u8]) -> Self;
}

macro_rules! create_interpret {
  ($primitive:ty) => {
    impl Interpret for $primitive {
      fn from_bytes (src: &[u8]) -> Self {
        <$primitive>::from_le_bytes(src.try_into().expect("slice with incorrect length"))
      }
    }
  }
}

create_interpret!(u8);
create_interpret!(u16);
create_interpret!(u32);
create_interpret!(i8);
create_interpret!(i16);
create_interpret!(i32);

create_interpret!(f32);
create_interpret!(f64);

impl Interpret for String {
  fn from_bytes (src: &[u8]) -> Self {
    let vector = Vec::from(src);
    let string = String::from_utf8(vector);
    string.unwrap()
  }
}
