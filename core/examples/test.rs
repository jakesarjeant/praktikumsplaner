use comfy_table::Table;
use planner_core::{generate, FachGewichtung};
use std::{fs::File, io::Read};
use tracing::{level_filters::LevelFilter, Level};
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

  tracing_subscriber::fmt()
    .with_max_level(LevelFilter::INFO)
    .init();

  let solution = generate(
    &plan,
    &vec![
      FachGewichtung {
        kuerzel: "M".to_string(),
        gewicht: 1.0,
      },
      FachGewichtung {
        kuerzel: "D".to_string(),
        gewicht: 1.0,
      },
    ],
  );

  println!("\nLösung: {solution:?}");

  // let mut table = Table::new();

  // table.set_header(vec!["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"]);

  // for day in
}
