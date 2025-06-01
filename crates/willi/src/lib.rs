#![doc = include_str!("../README.md")]

use std::{default::Default, io::Cursor, num::ParseIntError, str::FromStr};

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
  #[error("Ungültige Zeitangabe")]
  BadTime,
  #[error("Ungültige Zahl")]
  BadNumber(#[from] ParseIntError),
}

#[derive(Debug, Clone)]
pub struct WilliDocument {
  pub header: WilliHeader,
  pub days: Vec<WilliDay>,
  // TODO: Is storing the timetable out-of-band really a good idea?
  pub default_timetable: Vec<WilliTimeSlot>,
  // TODO: Alternative Timetables
  pub teachers: Vec<WilliTeacher>,
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
      default_timetable: Default::default(),
      teachers: Default::default(),
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
      ("S", x) => self.parse_S(x, record),
      // ("MP", _) => todo!(),
      ("L", x) => self.parse_L(x, record),
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
      // TODO: Collect warnings, rather than emitting one per line
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
      .take_while(|(p, _)| p != &'X')
      .map(|(p, b)| WilliPeriod {
        kind: match p {
          'V' => WilliPeriodKind::V,
          'N' => WilliPeriodKind::N,
          'M' => WilliPeriodKind::M,
          'n' => WilliPeriodKind::M, // TODO: This is a guess. Verify that this is correct.
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

  fn parse_S(&mut self, index: usize, record: &StringRecord) -> Result<(), LineError> {
    if index <= self.default_timetable.len() {
      // TODO: At this point, the entire table is guaranteed to be invalid. Should we fail?
      warn!("Stunden in falscher Reihenfolge");
    }

    // NOTE: Really, the correct record length seems to be 5, but we currently ignore the last wo,
    // so this should be fine
    if record.len() < 3 {
      return Err(LineError::TooShort);
    }

    let (start, end) = record[2].split_once('-').ok_or(LineError::BadTime)?;

    // FIXME: Get times from actual time fields
    let start = start
      .split_once('.')
      .map(|(hour, min)| Ok::<_, ParseIntError>((hour.parse()?, min.parse()?)))
      .transpose()?
      .ok_or(LineError::BadTime)?;

    let end = end
      .split_once('.')
      .map(|(hour, min)| Ok::<_, ParseIntError>((hour.parse()?, min.parse()?)))
      .transpose()?
      .ok_or(LineError::BadTime)?;

    self.default_timetable.push(WilliTimeSlot { start, end });

    Ok(())
  }

  fn parse_L(&mut self, index: usize, record: &StringRecord) -> Result<(), LineError> {
    if index <= self.default_timetable.len() {
      warn!("Lehrer in falscher Reihenfolge");
    }

    if record.len() < 6 {
      return Err(LineError::TooShort);
    }

    self.teachers.push(WilliTeacher {
      kuerzel: record[1].to_string(),
      kurzname: record[2].to_string(),
      name: record[3].to_string(),
      vorname: record[4].and_then(ToString::to_string),
      anrede: record[5].and_then(ToString::to_string),
      // Yes, the fields are out of order
      funktion: match &record[13] {
        "P" => Some(WilliTeacherFunction::P),
        "D" => Some(WilliTeacherFunction::D),
        "S" => Some(WilliTeacherFunction::S),
        "R" => Some(WilliTeacherFunction::R),
        "" => None,
        f => {
          warn!("Funktion {f} nicht erkannt");
          None
        }
      },
      // TODO: Is 0 really the correct default for all of these?
      sollwochenstunden: record[11].parse().unwrap_or(0),
      luecken: record[14].parse().unwrap_or(0),
      gew_block: record[17].parse().unwrap_or(0),
      gew_verteilung: record[18].parse().unwrap_or(0),
      gew_frueh: record[19].parse().unwrap_or(0),
      gew_spaet: record[20].parse().unwrap_or(0),
      max_hohlstunden: record[21].parse().unwrap_or(0),
      max_aufsichten: record[22].parse().unwrap_or(0),
      nachmittag_beruecksichtigen: &record[23] == "N",
      max_stundenzahl: record[28].parse().unwrap_or(0),
      max_verfuegungsstd: record[29].parse().unwrap_or(0),
      max_nachmittag: record[33].parse().unwrap_or(0),
    });

    Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct WilliHeader {
  pub version: usize,
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

/// Represents the start and end of a period in the timetable. Times are represented as tuples of
/// `(hour, minute)`, to avoid bringing chrono into play.
#[derive(Debug, Clone)]
pub struct WilliTimeSlot {
  pub start: (u8, u8),
  pub end: (u8, u8),
}

#[derive(Debug, Clone)]
pub struct WilliTeacher {
  /// Up to 5 chars
  pub kuerzel: String,
  /// Up to 7 chars
  pub kurzname: String,
  /// Up to 40 chars
  pub name: String,
  /// Up to 40 chars
  pub vorname: Option<String>,
  pub anrede: Option<String>,
  // NOTE: 5 unknown fields
  pub sollwochenstunden: usize,
  // NOTE: 1 unknown field
  pub funktion: Option<WilliTeacherFunction>,
  pub luecken: usize,
  // NOTE: 2 unknown fields
  pub gew_block: usize,
  pub gew_verteilung: usize,
  pub gew_frueh: usize,
  pub gew_spaet: usize,
  pub max_hohlstunden: usize,
  pub max_aufsichten: usize,
  pub nachmittag_beruecksichtigen: bool,
  // NOTE: 4 unknown fields
  pub max_stundenzahl: usize,
  pub max_verfuegungsstd: usize,
  // NOTE: 3 unknown fields
  pub max_nachmittag: usize,
  // TODO: Rest of the fields
}

#[derive(Debug, Clone)]
pub enum WilliTeacherFunction {
  /// Personalrat
  P,
  /// Direktorat
  D,
  /// Seminarlehrer
  S,
  /// Referendar
  R,
}

trait IfNonEmptyExt {
  fn and_then<T, F: FnOnce(&Self) -> T>(&self, f: F) -> Option<T>;

  fn or(&self, default: &'static str) -> &Self;
}

impl IfNonEmptyExt for str {
  fn and_then<T, F: FnOnce(&Self) -> T>(&self, f: F) -> Option<T> {
    (!self.is_empty()).then(|| f(self))
  }

  fn or(&self, default: &'static str) -> &Self {
    (!self.is_empty()).then_some(self).unwrap_or(default)
  }
}
