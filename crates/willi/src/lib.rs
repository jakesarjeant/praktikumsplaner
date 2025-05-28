#![doc = include_str!("../README.md")]

use std::{collections::HashMap, default::Default, io::Cursor, str::FromStr};

use csv::StringRecord;
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
}

pub struct WilliDocument {
  header: WilliHeader,
  days: Vec<WilliDay>,
}

impl WilliDocument {
  fn parse(source: String) -> Result<WilliDocument, DocumentError> {
    let Some((raw_header, body)) = source.split_once("\r\n") else {
      return Err(DocumentError::MissingHeader);
    };

    let header = raw_header.parse()?;
    let mut document = WilliDocument {
      header,
      days: Default::default(),
    };
    let mut line_errors = vec![];

    let mut csv_reader = csv::Reader::from_reader(Cursor::new(body));
    let mut records = csv_reader.records();

    while let Some(next_record) = records.next() {
      let record = next_record?;

      document
        .parse_record(&record)
        .or_else(|e| Err(line_errors.push((record, e))))
        .ok();
    }

    Ok(document)
  }

  fn parse_record(&mut self, record: &StringRecord) -> Result<(), LineError> {
    let Some(type_col) = record.get(1) else {
      return Err(LineError::MissingType);
    };

    let (typ, id): (&str, usize) = type_col
      .split_once(|c: char| !c.is_alphabetic())
      .map(|(typ, id)| Ok((typ, id.parse().map_err(|_| LineError::BadId)?)))
      .transpose()?
      .unwrap_or((type_col, 0));

    match (typ, id) {
      ("W", _) => todo!(),
      ("WP", _) => todo!(),
      ("T", x) => todo!(),
      ("S", x) => todo!(),
      ("MP", _) => todo!(),
      ("L", x) => todo!(),
      ("LB", x) => todo!(),
      ("R", x) => todo!(),
      ("G", x) => todo!(),
      ("F", x) => todo!(),
      ("K", x) => todo!(),
      ("X", x) => todo!(),
      ("O", x) => todo!(),
      ("Z", x) => todo!(),
      ("A", x) => todo!(),
      ("AV", x) => todo!(),
      ("J", x) => todo!(),
      ("U", x) => todo!(),
      ("PL", _) => todo!(),
      ("PLS", _) => todo!(),
      ("PKS", _) => todo!(),
      ("PRS", _) => todo!(),
      (typ, _) => Ok(warn!("Unbekannter Zeilentyp: {typ}")),
    }
  }
}

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

pub struct WilliDay {
  short: String,
  long: String,
  // TODO: Figure out exactly what the periods string means and find a better representation
  periods: String,
  // TODO: Encode the breaks into the representation of periods
  breaks: String,
}
