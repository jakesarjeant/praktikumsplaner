use std::{fs::File, io::Read};
use planner_core::{generate, FachGewichtung};
use willi::WilliStundenplan;

fn main() {
  let mut raw_source = vec![];
  File::open("../../WilliFiles/Stundenplandateien/AKG2024-25_31.03.BAL")
    .unwrap()
    .read_to_end(&mut raw_source)
    .unwrap();
  let reencoded_source: String = raw_source.iter().map(|&c| c as char).collect();

  let (plan, errors) = WilliStundenplan::parse(&reencoded_source[..]);

  if !errors.is_empty() {
    eprintln!("Encountered parse errors:");
    for error in errors {
      println!("{}", error.1);
    }
  }

  let solution = generate(
    &plan,
    &vec![
      FachGewichtung {
        kuerzel: "M".to_string(),
        gewicht: 1.0,
      },
      FachGewichtung {
        kuerzel: "Ph".to_string(),
        gewicht: 1.0,
      }
    ]
  );

  println!("Solution: {:?}", solution);
}
