#![doc = include_str!("../README.md")]

use std::{collections::HashMap, default::Default, io::Cursor, str::FromStr};

use csv::{ReaderBuilder, StringRecord};
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum DocumentError {
  #[error("Kopfzeile fehlt")]
  MissingHeader,
  #[error("Kopfzeile nicht erkannt")]
  InvalidHeader,
  #[error("Fehlerhafte CSV-Daten. Möglicherweise ist die Datei korrupt.")]
  CSV(#[from] csv::Error),
  #[error("Fehlerhafte Zeilen")]
  // TODO: Display the individual sub-errors
  BadLines(Vec<(StringRecord, LineError)>),
}

#[derive(Debug, Error)]
pub enum LineError {
  #[error("Zeile ohne Datentyp")]
  MissingType,
  #[error("Ungültiger Primärschlüssel")]
  BadId,
  #[error("Ungültige Zeile mit fehlenden Einträgen")]
  TooShort,
}

#[derive(Debug, Clone)]
pub struct WilliDocument {
  pub header: WilliHeader,
  pub days: Vec<WilliDay>,
}

impl FromStr for WilliDocument {
  type Err = DocumentError;

  fn from_str(source: &str) -> Result<WilliDocument, Self::Err> {
    let Some((raw_header, body)) = source.split_once("\r\n") else {
      return Err(DocumentError::MissingHeader);
    };

    let header = raw_header.parse()?;
    let mut document = WilliDocument {
      header,
      days: Default::default(),
    };
    let mut line_errors = vec![];

    let mut csv_reader = ReaderBuilder::new()
      .has_headers(false)
      .flexible(true)
      .from_reader(Cursor::new(body));
    let mut records = csv_reader.records();

    while let Some(next_record) = records.next() {
      let record = next_record?;

      document
        .parse_record(&record)
        .or_else(|e| Err(line_errors.push((record, e))))
        .ok();
    }

    if line_errors.is_empty() {
      Ok(document)
    } else {
      Err(DocumentError::BadLines(line_errors))
    }
  }
}

impl WilliDocument {
  fn parse_record(&mut self, record: &StringRecord) -> Result<(), LineError> {
    let Some(type_col) = record.get(0) else {
      return Err(LineError::MissingType);
    };

    let (typ, id): (&str, usize) = type_col
      .split_at_checked(
        type_col
          .find(|c: char| !c.is_alphabetic())
          .unwrap_or(type_col.len()),
      )
      .map(|(typ, id)| (typ, id.parse().unwrap_or(0)))
      .unwrap_or((type_col, 0));

    match (typ, id) {
      // ("w", _) => todo!(),
      // ("WP", _) => todo!(),
      ("T", x) => self.parse_T(x, record),
      // ("S", x) => todo!(),
      // ("MP", _) => todo!(),
      // ("L", x) => todo!(),
      // ("LB", x) => todo!(),
      // ("R", x) => todo!(),
      // ("G", x) => todo!(),
      // ("F", x) => todo!(),
      // ("K", x) => todo!(),
      // ("X", x) => todo!(),
      // ("O", x) => todo!(),
      // ("Z", x) => todo!(),
      // ("A", x) => todo!(),
      // ("AV", x) => todo!(),
      // ("J", x) => todo!(),
      // ("U", x) => todo!(),
      // ("PL", _) => todo!(),
      // ("PLS", _) => todo!(),
      // ("PKS", _) => todo!(),
      // ("PRS", _) => todo!(),
      (typ, _) => Ok(warn!("Unbekannter Zeilentyp '{typ}' wurde ignoriert")),
    }
  }

  fn parse_T(&mut self, index: usize, record: &StringRecord) -> Result<(), LineError> {
    // TODO: Figure out whether indexing is significant
    // My guess is that the indexes are only used to make sure elements are serialized in the same order
    if index <= self.days.len() {
      warn!("Tage in falscher Reihenfolge");
    }
    if record.len() < 5 {
      return Err(LineError::TooShort);
    }

    let periods = record[3]
      .chars()
      .zip(record[4].chars())
      .take_while(|(p, b)| p != &'X')
      .map(|(p, b)| WilliPeriod {
        kind: match p {
          'V' => WilliPeriodKind::V,
          'N' => WilliPeriodKind::N,
          'M' => WilliPeriodKind::M,
          _ => WilliPeriodKind::Unknown,
        },
        break_before: b == 'P',
      })
      .collect();

    let day = WilliDay {
      short: record[1].to_string(),
      long: record[2].to_string(),
      periods,
    };

    self.days.push(day);

    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct WilliHeader {
  version: usize,
}

impl FromStr for WilliHeader {
  type Err = DocumentError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split(' ').skip(3);

    let Some("Version:") = parts.next() else {
      return Err(DocumentError::InvalidHeader);
    };

    let Some(raw_version) = parts.next() else {
      return Err(DocumentError::InvalidHeader);
    };

    let version = raw_version
      .parse()
      .map_err(|_| DocumentError::InvalidHeader)?;

    Ok(WilliHeader { version })
  }
}

#[derive(Debug, Clone)]
pub struct WilliDay {
  /// Two-letter short code
  pub short: String,
  /// Full name of the day
  pub long: String,
  pub periods: Vec<WilliPeriod>,
}

#[derive(Debug, Clone)]
pub struct WilliPeriod {
  pub kind: WilliPeriodKind,
  /// Whether this period is preceded by a short break
  pub break_before: bool,
}

#[derive(Debug, Clone)]
pub enum WilliPeriodKind {
  /// "Vormittag"
  V,
  /// "Nachmittag"
  N,
  /// "Mittagspause"
  M,
  Unknown,
}
