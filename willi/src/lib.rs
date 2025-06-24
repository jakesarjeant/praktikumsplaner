use std::{io::Cursor, str::FromStr};

use csv::{Position, ReaderBuilder, StringRecord};
use serde::Deserialize;
use thiserror::Error;
use tracing::{debug, error};
use wasm_bindgen::prelude::*;

#[derive(Debug, Error)]
pub enum ParseError {
  #[error("Ungültige Kopfzeile — Keine Versionsdaten verfügbar.")]
  InvalidHeader,
  #[error("Lesen Abgebrochen — möglicherweise ist die Datei ungültig kodiert")]
  Aborted,
  #[error("Formatfehler — Zeile ohne Datentyp")]
  MissingType,
  #[error("Ungültige Datenzeile — {0}")]
  BadLine(csv::Error),
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
  pub fn parse(source: &str) -> (WilliStundenplan, Vec<(Option<Position>, ParseError)>) {
    let mut errors = vec![];

    let (raw_header, body) = source
      .split_once("\r\n")
      .map(|(h, b)| (Some(h), b))
      .unwrap_or((None, source));

    let header = raw_header.map(str::parse).transpose().unwrap_or_else(|e| {
      errors.push((Some(Position::new()), e));
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

    for result in csv_reader.records() {
      let record = match result {
        Ok(rec) => rec,
        Err(err) => {
          errors.push((err.position().cloned(), ParseError::BadLine(err)));
          continue;
        }
      };

      // Extract prefix letters and ID.
      // This parses the first field, i.e. "LC12" => ("LC", 12)
      let (typ, id): (String, usize) = {
        // TODO: Correctly handle "TRnnzz" lines (double primary key)
        let type_col = match record.get(0) {
          Some(c) => c,
          None => {
            errors.push((record.position().cloned(), ParseError::MissingType));
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

      // So that each invocation can have a different target type. Needs to be a macro due to continue.
      macro_rules! deserialize {
        ($rec:ident) => {
          match $rec.deserialize(None) {
            Ok(record) => record,
            Err(err) => {
              errors.push((
                err.position().or(record.position()).cloned(),
                ParseError::BadLine(err),
              ));
              continue;
            }
          }
        };
      }

      match (&typ[..], id) {
        ("W", _) => plan.schuldaten = Some(deserialize!(record)),
        _ => {}
      }
    }

    (plan, errors)
  }
}

#[wasm_bindgen]
impl WilliStundenplan {
  #[wasm_bindgen(getter)]
  pub fn willi_version(&self) -> Option<usize> {
    self.header.as_ref().map(|h| h.version)
  }
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
#[wasm_bindgen(js_name = WilliParseError)]
pub struct WasmError {
  pub byte: Option<u64>,
  pub line: Option<u64>,
  pub record: Option<u64>,
  #[wasm_bindgen(getter_with_clone)]
  pub error: String,
}

#[wasm_bindgen]
pub struct ParseResult {
  #[wasm_bindgen(getter_with_clone)]
  pub plan: WilliStundenplan,
  #[wasm_bindgen(getter_with_clone)]
  pub errors: Vec<WasmError>,
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(js_name = "parse_plan")]
pub fn wasm_parse_plan(source: String) -> ParseResult {
  let (plan, errors) = WilliStundenplan::parse(&source);

  ParseResult {
    plan,
    errors: errors
      .iter()
      .map(|(p, error)| WasmError {
        byte: p.as_ref().map(Position::byte),
        line: p.as_ref().map(Position::line),
        record: p.as_ref().map(Position::record),
        error: format!("{}", error),
      })
      .collect(),
  }
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
  #[serde(default)]
  pub titel1: Option<String>,
  #[serde(default)]
  pub titel2: Option<String>,
  #[serde(default)]
  pub schulnummer: Option<usize>,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  // print pretty errors in wasm https://github.com/rustwasm/console_error_panic_hook
  // This is not needed for tracing_wasm to work, but it is a common tool for getting proper error line numbers for panics.
  console_error_panic_hook::set_once();

  // Add this line:
  wasm_tracing::set_as_global_default();

  Ok(())
}
