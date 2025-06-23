use std::{io::Cursor, str::FromStr};

use csv::{Position, ReaderBuilder, StringRecord};
use serde::Deserialize;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[derive(Debug, Error)]
pub enum ParseError {
  #[error("Ungültige Kopfzeile — Keine Versionsdaten verfügbar.")]
  InvalidHeader,
  #[error("Lesen Abgebrochen — möglicherweise ist die Datei ungültig Kodiert")]
  Aborted,
  #[error("Formatfehler — Zeile ohne Datentyp")]
  MissingType,
  #[error("Ungültige Datenzeile — {0}")]
  Deserialize(csv::Error),
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct WilliStundenplan {
  /// Kopfzeile mit WILLI-Version, sofern eine gültige Kopfzeile vorgefunden wurde.
  header: Option<WilliHeader>,
  /// Schuldaten
  schuldaten: Option<SchuldatenZeile>,
}

impl WilliStundenplan {
  pub fn parse(source: &str) -> (WilliStundenplan, Vec<(Position, ParseError)>) {
    let mut errors = vec![];

    let (raw_header, body) = source
      .split_once("\r\n")
      .map(|(h, b)| (Some(h), b))
      .unwrap_or((None, source));

    let header = raw_header.map(str::parse).transpose().unwrap_or_else(|e| {
      errors.push((Position::new(), e));
      None
    });

    let mut plan = WilliStundenplan {
      header,
      schuldaten: None,
    };

    let mut csv_reader = ReaderBuilder::new()
      .has_headers(false)
      .flexible(true)
      .from_reader(Cursor::new(body));

    let mut peek_record = StringRecord::new();
    while !csv_reader.is_done() {
      // Peek record to determine prefix
      let Ok(true) = csv_reader.read_record(&mut peek_record) else {
        errors.push((csv_reader.position().clone(), ParseError::Aborted));
        break;
      };

      // Extract prefix letters and ID.
      // This parses the first field, i.e. "LC12" => ("LC", 12)
      let (typ, id): (String, usize) = {
        // TODO: Correctly handle "TRnnzz" lines (double primary key)
        let type_col = match peek_record.get(0) {
          Some(c) => c,
          None => {
            errors.push((csv_reader.position().clone(), ParseError::MissingType));
            continue;
          }
        };

        type_col
          .split_at_checked(
            type_col
              .find(|c: char| !c.is_alphabetic())
              .unwrap_or(type_col.len()),
          )
          .map(|(typ, id)| (typ.into(), id.parse().unwrap_or(0)))
          .unwrap_or((type_col.into(), 0))
      };

      let record = match peek_record.deserialize(None) {
        Ok(record) => record,
        Err(err) => {
          errors.push((
            err.position().unwrap_or(csv_reader.position()).clone(),
            ParseError::Deserialize(err),
          ));
          continue;
        }
      };

      match (&typ[..], id) {
        ("W", _) => plan.schuldaten = Some(record),
        _ => todo!(),
      }
    }

    (plan, errors)
  }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
#[wasm_bindgen(js_name = WilliParseError)]
pub struct WasmError {
  pub byte: u64,
  pub line: u64,
  pub record: u64,
  #[wasm_bindgen(getter_with_clone)]
  pub error: String,
}

#[wasm_bindgen]
pub struct ParseResult(
  #[wasm_bindgen(getter_with_clone)] pub WilliStundenplan,
  #[wasm_bindgen(getter_with_clone)] pub Vec<WasmError>,
);

#[cfg(target_family = "wasm")]
#[wasm_bindgen(js_name = "parse_plan")]
pub fn wasm_parse_plan(source: String) -> ParseResult {
  let (plan, errors) = WilliStundenplan::parse(&source);

  ParseResult(
    plan,
    errors
      .iter()
      .map(|(p, error)| WasmError {
        byte: p.byte(),
        line: p.line(),
        record: p.record(),
        error: format!("{}", error),
      })
      .collect(),
  )
}

#[derive(Debug, Clone)]
pub struct WilliHeader {
  pub version: usize,
}

impl FromStr for WilliHeader {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split(' ').skip(3);

    let Some("Version:") = parts.next() else {
      return Err(ParseError::InvalidHeader);
    };

    let Some(raw_version) = parts.next() else {
      return Err(ParseError::InvalidHeader);
    };

    let version = raw_version.parse().map_err(|_| ParseError::InvalidHeader)?;

    Ok(WilliHeader { version })
  }
}

//// WILLI TABLES ////

// W-Zeile
#[derive(Debug, Clone, Deserialize)]
pub struct SchuldatenZeile {
  pub schulname: String,
  pub titel1: Option<String>,
  pub titel2: Option<String>,
  pub schulnummer: Option<usize>,
}
