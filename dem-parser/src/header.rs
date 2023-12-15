use super::provider::BinaryProvider;
use super::common::ParseError;

#[derive(Default)]
pub struct DemoHeader {
  pub header_magic: String,
  pub demo_version: u32,
  pub net_version: u32,
  pub server_name: String,
  pub client_name: String,
  pub map_name: String,
  pub game_dir: String,
  pub demo_seconds: f32,
  pub demo_ticks: u32,
  pub demo_frames: u32,
  pub sign_on_length: u32,
}

impl DemoHeader {
  fn new () -> Self {
    Default::default()
  }
}

pub fn parse_header<'a, T: BinaryProvider<'a>> (reader: &mut T)
  -> Result<DemoHeader, ParseError>
{
  let mut header = DemoHeader::new();
  header.header_magic = reader.read_str(8)?;

  if header.header_magic != "HL2DEMO\0" {
    return Err(ParseError::InvalidFormat("wrong magic".to_string()));
  }

  header.demo_version = reader.read_as()?;
  header.net_version = reader.read_as()?;
  header.server_name = reader.read_str(260)?;
  header.client_name = reader.read_str(260)?;
  header.map_name = reader.read_str(260)?;
  header.game_dir = reader.read_str(260)?;
  header.demo_seconds = reader.read_as()?;
  header.demo_ticks = reader.read_as()?;
  header.demo_frames = reader.read_as()?;
  header.sign_on_length = reader.read_as()?;
  Ok(header)
}
