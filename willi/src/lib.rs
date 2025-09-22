use std::{collections::HashMap, io::Cursor, str::FromStr};

use csv::{Position, ReaderBuilder, StringRecord};
use js_sys::{Object, Reflect};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;
use tracing::{debug, error, warn};
use wasm_bindgen::prelude::*;

#[derive(Debug, Error)]
pub enum ParseError {
  #[error("Ungültige Kopfzeile — Keine Versionsdaten verfügbar.")]
  InvalidHeader,
  #[error("Lesen Abgebrochen — möglicherweise ist die Datei ungültig kodiert")]
  Aborted,
  #[error("Formatfehler — Zeile ohne Datentyp")]
  MissingType,
  #[error("Ungültige Datenzeile — {0}\n\tZeile: {1:?}")]
  BadLine(csv::Error, Option<csv::StringRecord>),
}

#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WilliStundenplan {
  /// Kopfzeile mit WILLI-Version, sofern eine gültige Kopfzeile vorgefunden wurde.
  header: Option<WilliHeader>,
  /// Schuldaten
  schuldaten: Option<SchuldatenZeile>,
  /// Fächer
  faecher: SparseVec<FachZeile>,
  /// Unterrichtseinheit
  unterrichtseinheiten: SparseVec<UnterrichtsZeile>,
  /// Stunden im Lehrerplan
  stunden_lehrerplan: Vec<LehrerStundenZeile>,
  /// Klassen
  klassen: SparseVec<KlassenZeile>,
  /// Tage
  tage: SparseVec<TagZeile>,
  /// Stunden
  stunden: SparseVec<StundenZeile>,
  /// Lehrkräfte
  lehrkraefte: SparseVec<LehrkraftZeile>,
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
      faecher: Default::default(),
      unterrichtseinheiten: Default::default(),
      klassen: Default::default(),
      stunden_lehrerplan: vec![],
      tage: Default::default(),
      stunden: Default::default(),
      lehrkraefte: Default::default(),
    };

    let mut csv_reader = ReaderBuilder::new()
      .has_headers(false)
      .flexible(true)
      .from_reader(Cursor::new(body));

    for result in csv_reader.records() {
      let record = match result {
        Ok(rec) => rec,
        Err(err) => {
          errors.push((err.position().cloned(), ParseError::BadLine(err, None)));
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
                ParseError::BadLine(err, Some($rec)),
              ));
              continue;
            }
          }
        };
      }

      macro_rules! overwrite_warn {
        ($expr:expr) => {
          if let Some(old) = $expr {
            warn!("Record {id} was overwritten! Old record: {:?}", old)
          }
        };
      }

      match (&typ[..], id) {
        ("W", _) => {
          plan.schuldaten = Some(deserialize!(record));
        }
        ("F", id) => overwrite_warn!(plan.faecher.insert(id, deserialize!(record))),
        ("U", id) => overwrite_warn!(plan.unterrichtseinheiten.insert(id, deserialize!(record))),
        ("PL", _) => plan.stunden_lehrerplan.push(deserialize!(record)),
        ("K", id) => overwrite_warn!(plan.klassen.insert(id, deserialize!(record))),
        ("T", id) => overwrite_warn!(plan.tage.insert(id, deserialize!(record))),
        ("S", id) => overwrite_warn!(plan.stunden.insert(id, deserialize!(record))),
        ("L", id) => overwrite_warn!(plan.lehrkraefte.insert(id, deserialize!(record))),
        _ => {}
      }
    }

    (plan, errors)
  }

  pub fn klassen(&self) -> &SparseVec<KlassenZeile> {
    &self.klassen
  }

  pub fn tage(&self) -> &SparseVec<TagZeile> {
    &self.tage
  }

  pub fn lehrerstunden(&self) -> &Vec<LehrerStundenZeile> {
    &self.stunden_lehrerplan
  }

  pub fn stunden(&self) -> &SparseVec<StundenZeile> {
    &self.stunden
  }
}

// Yes, this is probably slow... Better idea?
macro_rules! to_js_object {
  ($data:expr) => {{
    let obj = Object::new();

    for (key, value) in $data {
      let js_key = JsValue::from_str(&key.to_string());
      let js_val = value.clone().into();
      let id_key = JsValue::from_str("id");
      Reflect::set(&js_val, &id_key, &js_key).expect("Failed to set ID");
      Reflect::set(&obj, &js_key, &js_val).expect("Failed to update map");
    }

    obj.into()
  }};
}

#[wasm_bindgen]
impl WilliStundenplan {
  #[wasm_bindgen(getter)]
  pub fn willi_version(&self) -> Option<usize> {
    self.header.as_ref().map(|h| h.version)
  }

  #[wasm_bindgen(getter, unchecked_return_type = "{[id:string]:FachZeile}")]
  pub fn faecher(&self) -> JsValue {
    to_js_object!(self.faecher.iter())
  }

  #[wasm_bindgen(getter, unchecked_return_type = "{[id:string]:UnterrichtsZeile}")]
  pub fn unterrichte(&self) -> JsValue {
    to_js_object!(self.unterrichtseinheiten.iter())
  }

  #[wasm_bindgen(getter, unchecked_return_type = "{[id:string]:LehrkraftZeile}")]
  pub fn lehrkraefte(&self) -> JsValue {
    to_js_object!(self.lehrkraefte.iter())
  }

  pub fn stunden_lehrerplan(&self) -> Vec<LehrerStundenZeile> {
    self.stunden_lehrerplan.clone()
  }

  pub fn to_js(&self) -> JsValue {
    serde_wasm_bindgen::to_value(self).unwrap()
  }

  pub fn from_js(value: JsValue) -> Self {
    // TODO: Error handling?
    serde_wasm_bindgen::from_value(value).unwrap()
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchuldatenZeile {
  #[allow(dead_code)]
  id: String,
  pub schulname: String,
  #[serde(default)]
  pub titel1: Option<String>,
  #[serde(default)]
  pub titel2: Option<String>,
  #[serde(default)]
  pub schulnummer: Option<usize>,
}

// TODO: WP
// TODO: WI

// T-Zeile
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct TagZeile {
  #[allow(dead_code)]
  id: String,
  #[wasm_bindgen(getter_with_clone)]
  pub kurz: String,
  #[wasm_bindgen(getter_with_clone)]
  pub lang: String,
  // TODO: Parse field
  #[wasm_bindgen(getter_with_clone)]
  pub stundenmerkmale: String,
  // TODO: Parse field
  #[wasm_bindgen(getter_with_clone)]
  pub pausen: String,
  // TODO: Parse field
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub stundenzeiten: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct StundenZeile {
  #[allow(dead_code)]
  id: String,
  #[wasm_bindgen(getter_with_clone)]
  pub kurz: String,
  #[wasm_bindgen(getter_with_clone)]
  pub lang: String,
  // TODO: Parse field
  #[wasm_bindgen(getter_with_clone)]
  pub von: String,
  // TODO: Parse field
  #[wasm_bindgen(getter_with_clone)]
  pub bis: String,
}

// TODO: TRnnzz
// TODO: Qnn
// TODO: MP
// TODO: Enn

// L-Zeile
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct LehrkraftZeile {
  #[allow(dead_code)]
  id: String,
  #[wasm_bindgen(getter_with_clone)]
  pub kuerzel: String,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub kurz: Option<String>,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub name: Option<String>,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub vorname: Option<String>,
  // TODO: Parse field
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub anrede: Option<String>,
  // #[serde(default)]
  // #[serde(
  //   deserialize_with = "de_german_float",
  //   serialize_with = "ser_german_float"
  // )]
  // pub unterrichtspflichtzeit: Option<f64>,
  // #[serde(default)]
  // pub ermaessigungen: Option<f64>,
  // #[serde(default)]
  // pub anrechnungen: Option<f64>,
  // #[serde(default)]
  // pub effektive_stundenzahl: Option<f64>,
  // #[serde(default)]
  // pub arbeitszeitkonto: Option<f64>,
  // #[serde(default)]
  // #[wasm_bindgen(getter_with_clone)]
  // pub deputat: Option<String>,
  // // TODO: Parse
  // #[serde(default)]
  // #[wasm_bindgen(getter_with_clone)]
  // pub besonderheiten: Option<String>,
  // // TODO: Parse
  // #[serde(default)]
  // #[wasm_bindgen(getter_with_clone)]
  // pub funktion: Option<String>,
  // // TODO: Rest of fields
}

// TODO: LBnn
// TODO: LCnn
// TODO: LQnn
// TODO: LGnn
// TODO: Rnn
// TODO: RQnn
// TODO: RGnn
// TODO: Gnn

// F-Zeile
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct FachZeile {
  #[allow(dead_code)]
  id: String,
  #[wasm_bindgen(getter_with_clone)]
  pub kuerzel: String,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub kurz: Option<String>,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub name: Option<String>,
  #[serde(default)]
  // #[wasm_bindgen(getter_with_clone)]
  pub merkmal: Option<usize>,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub eigenschaft: Option<FachEigenschaft>,
  #[serde(default)]
  pub konzentration: Option<Konzentration>,
  #[serde(default)]
  pub wertung: Option<Wertung>,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub fachgruppe: Option<String>,
  #[serde(default)]
  pub fachraumgruppe: Option<usize>,
  // TODO: Was heißt "Ganzzahl im RGB-Format?"
  #[serde(default)]
  pub farbe: Option<usize>,
  #[serde(default)]
  pub fakultasfilter: Option<usize>,
  // TODO: Proper enum representation
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub zeiteinschraenkungen: Option<String>,
  // TODO: Parse time filters
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub zeitfilter: Option<String>,
  // TODO: Does this use IDs ore kuerzel?
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  // pub fachkollision: Vec<usize>,
  pub fachkollision: String,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub km_fach: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[wasm_bindgen]
pub enum FachEigenschaft {
  /// Doppelstündiges Fach
  D,
  /// Bei Pool-Verplanung ignoriert
  I,
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Default)]
#[repr(u8)]
#[wasm_bindgen]
pub enum Konzentration {
  #[default]
  Minimum = 0,
  Niedrig = 1,
  Mittel = 2,
  Hoch = 3,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[wasm_bindgen]
pub enum Wertung {
  /// Wissenschaftlich
  W,
  /// Nichtwissenschaftlich
  N,
}

// TODO: FQnn
// TODO: CT
// TODO: C

// TODO: Knn
// K-Zeile
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct KlassenZeile {
  #[allow(dead_code)]
  id: String,
  #[wasm_bindgen(getter_with_clone)]
  pub kuerzel: String,
  #[wasm_bindgen(getter_with_clone)]
  pub kurz: Option<String>,
  #[wasm_bindgen(getter_with_clone)]
  pub name: Option<String>,
  /// "Kürzel des Klassenraums"
  #[wasm_bindgen(getter_with_clone)]
  pub klassenraum: Option<String>,
  #[wasm_bindgen(getter_with_clone)]
  pub klassenleiter: Option<String>,
  #[wasm_bindgen(getter_with_clone)]
  pub zweitklassenleiter: Option<String>,
  pub deputat: Option<usize>,
  pub schuelerzahl: Option<usize>,
  pub weiblich: Option<usize>,
  pub jahrgangsstufe: Option<usize>,
  // TODO: Parse field
  #[wasm_bindgen(getter_with_clone)]
  pub besonderheiten: Option<String>,
  #[wasm_bindgen(getter_with_clone)]
  pub schultyp: Option<String>,
  // TODO: Was heißt "ganzzahl im RGB-Format?"
  #[wasm_bindgen(getter_with_clone)]
  pub farbe: Option<String>,
  #[wasm_bindgen(getter_with_clone)]
  pub stammklasse: Option<String>,
  pub mittagspause_min: Option<usize>,
  pub mittagspause_max: Option<usize>,
  pub nachmittag_max: Option<usize>,
  #[wasm_bindgen(getter_with_clone)]
  pub schule: Option<String>,
  /// Anzahl der katholischen Schüler in der Klasse
  pub rk: Option<usize>,
  /// Anzahl der evangelischen Schüler in der Klasse
  pub ev: Option<usize>,
  /// Anzahl der "sonstigen" Schüler in der Klasse (vermutlich bezogen auf Religion?)
  pub sonst: Option<usize>,
  /// Anzahl der Fahrschüler in der Klasse
  pub fahr: Option<usize>,
  pub zeitraster: Option<u8>,
  #[wasm_bindgen(getter_with_clone)]
  #[serde(default)]
  pub asv_klasse: Option<String>,
}

// TODO: KBnn
// TODO: KQnn
// TODO: KDnn
// TODO: KGnn
// TODO: Xnn
// TODO: Ynn
// TODO: Onn
// TODO: Znn
// TODO: Ann
// TODO: AVtt
// TODO: Jnn

// U-Zeile
#[derive(Debug, Clone, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct UnterrichtsZeile {
  #[allow(dead_code)]
  id: String,
  /// Kuerzel der Lehrkraft
  #[wasm_bindgen(getter_with_clone)]
  pub lehrkraft: String,
  /// Kuerzel des Fachs
  #[wasm_bindgen(getter_with_clone)]
  pub fach: String,
  /// Kuerzel der Klasse
  #[wasm_bindgen(getter_with_clone)]
  pub klasse: String,
  #[wasm_bindgen(getter_with_clone)]
  #[serde(default)]
  pub kopplung: Option<String>,
  #[serde(default)]
  pub stundenzahl: u8,
  #[serde(default)]
  pub stundenzahl_klasse: Option<usize>,
  #[serde(default)]
  pub stundenzahl_lehrer: Option<usize>,
  #[serde(default)]
  pub stundenzahl_fachraum: Option<usize>,
  #[serde(default)]
  pub schuelerzahl: Option<usize>,
  // PUB TODO: Proper enum/static type
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub besonderheiten: Option<String>,
  #[wasm_bindgen(getter_with_clone)]
  #[serde(default)]
  pub lehrerbezeichner: Option<String>,
  #[wasm_bindgen(getter_with_clone)]
  #[serde(default)]
  pub fachbezeichner: Option<String>,
  #[wasm_bindgen(getter_with_clone)]
  #[serde(default)]
  pub klassenbezeichner: Option<String>,
  #[serde(default)]
  pub fachraumgruppe: Option<usize>,
  /// Kuerzel des fest vorgesehenen Raums
  #[wasm_bindgen(getter_with_clone)]
  #[serde(default)]
  pub raum: Option<String>,
  /// Minimale anzahl Doppelstnden
  #[serde(default)]
  pub doppmin: Option<usize>,
  /// Maximale anzahl Doppelstnden
  #[serde(default)]
  pub doppmax: Option<usize>,
  #[serde(default)]
  pub blockgroesse: Option<usize>,
  #[serde(default)]
  pub max_pro_tag: Option<usize>,
  // PUB TODO: Proper enum/static type
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub doppelstundenparameter: Option<String>,
  // PUB TODO: Parse field
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub b_unterrichtseinheit: Option<String>,
  // PUB TODO: Parse field
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub zeiteinschraenkungen: Option<String>,
  // PUB TODO: Parse field
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub zeitfilter: Option<String>,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub bedingung: Option<String>,
  #[serde(default)]
  pub schuelerfilter: Option<usize>,
  #[serde(default)]
  pub wunschraster: Option<usize>,
  // PUB TODO: Parse field
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub unterrichtsphase: Option<String>,
  // PUB TODO: Restrict to 0..3
  #[serde(default)]
  pub gewicht_folgetage: Option<usize>,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub asv_unterrichtsart: Option<String>,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub asv_bereich: Option<String>,
}

// TODO: Bnn
// TODO: VLnn
// TODO: VSnn

// PL-Zeile
#[derive(Clone, Debug, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct LehrerStundenZeile {
  #[allow(dead_code)]
  id: String,
  #[wasm_bindgen(getter_with_clone)]
  pub tag_stunde: TagStunde,
  #[wasm_bindgen(getter_with_clone)]
  pub lehrkraft: String,
  #[wasm_bindgen(getter_with_clone)]
  pub klasse: String,
  #[wasm_bindgen(getter_with_clone)]
  pub fach: String,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub raum: Option<String>,
  #[serde(default)]
  #[wasm_bindgen(getter_with_clone)]
  pub fixierung: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[wasm_bindgen]
pub struct TagStunde {
  #[wasm_bindgen(getter_with_clone)]
  pub tag: String,
  #[wasm_bindgen(getter_with_clone)]
  pub stunde: String,
}

impl<'de> Deserialize<'de> for TagStunde {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let string = String::deserialize(deserializer)?;

    let Some((tag, stunde)) = string
      .split_once(" ")
      .map(|(t, s)| (t.to_string(), s.to_string()))
    else {
      return Err(serde::de::Error::custom("Ungültige Stundenzuordnung"));
    };

    Ok(TagStunde { tag, stunde })
  }
}

// TODO: PLS
// TODO: PKS
// TODO: PRS
// TODO: Dnn
// TODO: MK

//// UTILITY DATA STRUCTURES ////

// TODO: Option<Box<T>> to curb memory usage?
// TODO: Custom debug impl that collapses holes into e.g. <4 empty>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SparseVec<T>(Vec<Option<T>>);

impl<T> Default for SparseVec<T> {
  fn default() -> Self {
    SparseVec(vec![])
  }
}

impl<T> SparseVec<T> {
  pub fn insert(&mut self, idx: usize, val: T) -> Option<T> {
    if self.0.len() > idx {
      return self.0[idx].replace(val);
    }

    while self.0.len() < idx {
      self.0.push(None);
    }

    self.0.push(Some(val));
    return None;
  }

  pub fn get(&self, idx: usize) -> Option<&T> {
    self.0.get(idx).and_then(|x| x.as_ref())
  }

  pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
    self.0.get_mut(idx).and_then(|x| x.as_mut())
  }

  pub fn iter(&self) -> impl Iterator<Item = (usize, &T)> {
    self
      .0
      .iter()
      .enumerate()
      .filter_map(|(id, x)| x.as_ref().map(|x| (id, x)))
  }
}

pub fn de_german_float<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;
  let s = s.replace(',', ".");
  s.parse::<f64>().map_err(serde::de::Error::custom)
}

pub fn ser_german_float<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  // Format with a period first, then replace with comma
  let s = format!("{}", value).replace('.', ",");
  serializer.serialize_str(&s)
}
