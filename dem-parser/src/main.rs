use dem_parser::header::parse_header;
use dem_parser::readers::binary::BinaryReader;

fn print_header_info<'a> (content: &'a [u8]) {
  let mut reader = BinaryReader::from_bytes(&content);
  let parsed = parse_header(&mut reader);

  if let Ok(header) = parsed {
    println!("valid magic: {}", header.header_magic);
    println!("demo version: {}", header.demo_version);
    println!("net version: {}", header.net_version);
    println!("server name: {}", header.server_name);
    println!("client name: {}", header.client_name);
    println!("demo map name: {}", header.map_name);
    println!("game directory: {}", header.game_dir);
    println!("playback seconds: {}", header.demo_seconds);
    println!("number of ticks: {}", header.demo_ticks);
    println!("number of frames: {}", header.demo_frames);
    println!("sign-on data length: {}", header.sign_on_length);
  }
  else {
    println!("Something went wrong... details:");
    println!("{}", parsed.err().unwrap().to_string());
  }
}

pub fn main () {
  let contents: Vec<Vec<u8>> = vec![
    include_bytes!("samples/sample0_good.dem").to_vec(),
    include_bytes!("samples/sample1_bad_invalid.dem").to_vec(),
    include_bytes!("samples/sample2_bad_invalid.dem").to_vec(),
    include_bytes!("samples/sample3_bad_short.dem").to_vec(),
    include_bytes!("samples/sample4_bad_short.dem").to_vec(),
    include_bytes!("samples/sample5_bad_short.dem").to_vec(),
  ];

  for content in contents {
    println!("--- Demo info ---");
    print_header_info(&content);
    println!();
  }
}
