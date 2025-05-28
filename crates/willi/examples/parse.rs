use std::{fs::File, io::Read, path::PathBuf};

use willi::WilliDocument;

fn main() {
  let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("../../Willi/Stundenplandateien/AKG2024-25_31.03.BAL");

  let mut file = File::open(path).unwrap();

  let mut raw_source = vec![];
  file.read_to_end(&mut raw_source).unwrap();
  let reencoded_source: String = raw_source.iter().map(|&c| c as char).collect();

  let doc: WilliDocument = reencoded_source.parse().unwrap();

  println!("Extrahierter Stundenplan:");
  println!("{:#?}", doc);
}
