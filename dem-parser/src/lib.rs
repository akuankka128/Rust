pub mod interpret;
pub mod provider;
pub mod readers;
pub mod header;
pub mod common;

#[cfg(test)]
mod tests {
  use crate::provider::BinaryProvider;
  use readers::binary::BinaryReader;
  use super::*;

  fn get_sample_data () -> Vec<u8> {
    vec![ 1, 2, 3, 4 ]
  }

  fn get_zero_data () -> Vec<u8> {
    vec![ 0; 4 ]
  }

  #[test]
  fn read_test_u8 () {
    let data = get_sample_data();
    let mut read = BinaryReader::from_bytes(&data);
    let value = read.read_as::<u8>();
    assert_eq!(1u8, value.unwrap_or_default());
  }

  #[test]
  fn read_test_f32 () {
    let data = get_zero_data();
    let mut read = BinaryReader::from_bytes(&data);
    let value = read.read_as::<f32>();
    assert_eq!(0f32, value.unwrap_or_default());
  }

  #[test]
  #[should_panic]
  fn bounds_check_beyond () {
    let data = get_sample_data();
    let mut read = BinaryReader::from_bytes(&data);
    read.skip(69);
    read.read(1).unwrap();
  }

  #[test]
  #[should_panic]
  fn bounds_check_below () {
    let data = get_sample_data();
    let mut read = BinaryReader::from_bytes(&data);

    // The .back(1) call is actually the
    // first to panic due to an underflow
    read.back(1);
    read.read(1).unwrap();
  }

  #[test]
  fn seeking_bidi () {
    let data = get_sample_data();
    let mut read = BinaryReader::from_bytes(&data);
    read.skip(1);
    read.back(1);
    read.skip(69);
    read.back(69);
  }
}
