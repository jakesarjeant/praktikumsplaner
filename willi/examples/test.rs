use std::{fs::File, io::Read};

use willi::WilliStundenplan;

fn main() {
  let mut raw_source = vec![];
  File::open("/Users/jake/Downloads/AKG2025.BAL")
    .unwrap()
    .read_to_end(&mut raw_source)
    .unwrap();
  let reencoded_source: String = raw_source.iter().map(|&c| c as char).collect();

  let (_plan, errors) = WilliStundenplan::parse(&reencoded_source[..]);

  if !errors.is_empty() {
    eprintln!("Encountered parse errors:");
    for error in errors {
      println!("{}", error.1);
    }
  }
}
