use std::cell::LazyCell;

use ndarray::Array2;
use serde::Serialize;
use tracing::{debug, info, trace, warn};
use wasm_bindgen::prelude::*;
use wasm_tracing::WasmLayerConfig;
use willi::WilliStundenplan;

const GLOBAL: LazyCell<web_sys::DedicatedWorkerGlobalScope> =
  LazyCell::new(|| js_sys::global().unchecked_into());

#[wasm_bindgen]
#[derive(Debug)]
pub struct Problem {
  time_slots: usize,
  classes: usize,
  // [subject] = weight
  /// Weights must add up to 1
  subject_weights: Vec<f64>,
  // [class][slot] = Option<(subject, pl_index)>
  schedule: Array2<Option<(usize, usize)>>,
}

#[derive(Debug)]
pub struct Solution {
  // [slot] = Option<pl_index>
  assignments: Vec<Option<usize>>,
}

#[derive(Serialize)]
struct Progress {
  best: f64,
  current_cost: f64,
  current_classes: usize,
  visited: usize,
}

#[derive(Serialize)]
struct ProgressMessage {
  r#type: String,
  progress: Progress,
}

#[derive(Serialize)]
struct SolutionMessage {
  r#type: String,
  solution: Vec<Vec<Option<usize>>>
}

impl Problem {
  // Rekursiver Optimierungsalgorithmus
  fn search(
    &self,
    slot: usize,
    // Must be pre-filled with [time_slots] Nones
    current: &mut Vec<Option<usize>>,
    used_classes: &mut Vec<usize>,
    // [idx] = count; Where idx matches the index of that subject in `subject_weights`
    // must be pre-filled with zeroes
    subject_counts: &mut Vec<usize>,
    best: &mut Option<Solution>,
    best_cost: &mut f64,
    best_used_classes: &mut usize,
    nodes_visited: &mut usize,
    // Map used to finalize the intermediate plans
    timeslots: &Vec<(&str, usize, usize)>,
  ) {
    // Gewichtung der gleichmäßigen verteilung der fächer. 0 = Verteilung wird ignoriert
    const BALANCE_WT: f64 = 3.0;

    // Kostenfunktion
    fn cost(
      problem: &Problem,
      slot: usize,
      used_classes: &mut Vec<usize>,
      subject_counts: &mut Vec<usize>,
    ) -> f64 {
      // Strafe für Anzahl der Klassen
      let num_classes = used_classes.len() as f64;

      // Strafe für Abweichung von Gewichtung
      let imbalance = subject_counts
        .iter()
        .zip(problem.subject_weights.iter())
        .map(|(&count, weight)| {
          let target = problem.time_slots as f64 * weight;
          (count as f64 - target).abs() / target
        })
        .sum::<f64>();

      let cost = num_classes + (BALANCE_WT * imbalance);
      debug!(
        "Current branch cost: {cost} = {num_classes} + {} = {num_classes} + ({BALANCE_WT} * {imbalance})",
        BALANCE_WT * imbalance
      );

      cost
    }

    // Abbruchbedingung: letzte Stunde erreicht.
    if slot == self.time_slots {
      // Falls die Lösung eine Verbesserung darstellt: Speichern der neuen Lösung
      let current_cost = cost(self, slot, used_classes, subject_counts);
      info!(
        "Comparing current cost {current_cost} to best {best_cost}: {} incoming\t(bal {:?})",
        if current_cost < *best_cost {
          "accepted"
        } else {
          "rejected"
        },
        subject_counts
      );
      // TODO: Currently, this check will **always** pass, since any other branch would already be
      // pruned. It is purely a safety against future changes of the pruning heuristic.
      if current_cost < *best_cost {
        *best = Some(Solution {
          assignments: current.clone(),
        });
        *best_cost = current_cost;
        *best_used_classes = used_classes.len()
      }
      // Rekursionsabbruch
      return;
    }

    let mut assigned = false;

    let mut branches = 0.0;
    for class in 0..self.classes {
      // Falls diese Klasse zu dieser Zeit in einem Fach des Praktikanten unterrichtet wird
      if let Some((subject, pl_index)) = &self.schedule[[class, slot]] {
        // Klasse für diese Stunde eintragen
        current[slot] = Some(*pl_index);
        let was_new = if used_classes.contains(&class) {
          false
        } else {
          used_classes.push(class);
          true
        };

        // Verteilungen aktualisieren
        subject_counts[*subject] += 1;

        // Fortschritt Zurückmelden
        let current_cost = cost(self, slot, used_classes, subject_counts);
        let current_classes = used_classes.len();
        // Rate-limit progresse messages
        if *nodes_visited % 3197 == 0 {
          GLOBAL.post_message(
            &serde_wasm_bindgen::to_value(&ProgressMessage {
              r#type: "progress".to_string(),
              progress: Progress {
                best: *best_cost,
                current_cost,
                current_classes,
                visited: *nodes_visited,
              },
            })
            .unwrap(),
          );
        }

        if *nodes_visited % 1000000 == 0 {
          if let Some(solution) = best {
            GLOBAL.post_message(
              &serde_wasm_bindgen::to_value(&SolutionMessage {
                r#type: "solution".to_string(),
                solution: finalize(&solution, timeslots),
              }).unwrap()
            );
          }
        }

        // Nur weiter suchen, wenn diese Lösung nicht schon schlechter ist als die Letzte
        // if used_classes.len()  < *best_used_classes {
        if current_classes < *best_used_classes || current_cost < *best_cost {
          // Pruning heuristic
          // if cost(self, slot, used_classes, subject_counts) < *best_cost {
          // Weiter bei der nächsten Stunde
          branches += 1.0;
          self.search(
            slot + 1,
            current,
            used_classes,
            subject_counts,
            best,
            best_cost,
            best_used_classes,
            nodes_visited,
            timeslots
          );
        } else {
          debug!("Branch pruned (at slot {slot})");
        }

        // Backtracking: eintrag rückgänging machen
        current[slot] = None;
        subject_counts[*subject] -= 1;
        if was_new {
          used_classes.retain(|c| *c != class);
        }

        // Es gibt mindestens eine gültige Zuweisung dieser Stunde
        assigned = true;
      }
    }

    // Mit leerem Eintrag, falls es keine gültige Stunde gibt
    if !assigned {
      self.search(
        slot + 1,
        current,
        used_classes,
        subject_counts,
        best,
        best_cost,
        best_used_classes,
        nodes_visited,
        timeslots
      );
    }

    *nodes_visited += 1;
  }
}

#[wasm_bindgen]
pub struct FachGewichtung {
  #[wasm_bindgen(getter_with_clone)]
  pub kuerzel: String,
  #[wasm_bindgen(getter_with_clone)]
  pub gewicht: f64,
}

#[wasm_bindgen(unchecked_return_type = "(string | null)[][]")]
/// Siehe [`generate`].
pub fn wasm_generate(
  raw_plan: String,
  subjects: Vec<String>,
  weights: Vec<f64>,
  excluded_teachers: Vec<String>,
) -> JsValue {
  info!("Parsing!");
  let (plan, _errors) = WilliStundenplan::parse(&raw_plan);

  let subject_weights = subjects
    .iter()
    .cloned()
    .zip(weights.iter().copied().chain(std::iter::repeat(1.0)))
    .map(|(kuerzel, gewicht)| FachGewichtung { kuerzel, gewicht })
    .collect();

  let solution = generate(&plan, &subject_weights, &excluded_teachers);
  serde_wasm_bindgen::to_value(&solution).unwrap()
}

/// Erstellt einen Stundenplan
///
/// # Parameter
///
/// * `plan` — ein WILLI2-Stundenplan
/// * `subjects` — eine Liste von Fächerkürzeln im Plan, gepaart mit gewichtungen. Kein Fach darf
/// zweimal vorkommen.
///
/// # Rückgabe
/// Eine zwei-dimensionale Liste der Form `[tag][stunde] = [index]`. `index` indiziert die Tabelle
/// "Stunden im Lehrerplan" (PL) des übergebenen Plans. Theoretisch wäre eine einfache Liste von
/// Indizes ausreichend, so wird aber der Aufwand sie wieder in eine Tabelle umzubauen gespart.
pub fn generate(
  plan: &WilliStundenplan,
  subjects: &Vec<FachGewichtung>,
  excluded_teachers: &Vec<String>,
) -> Vec<Vec<Option<usize>>> {
  // NOTE: This assumes each subject only appears once.
  let classes: Vec<_> = plan.klassen().iter().collect();
  let days: Vec<_> = plan.tage().iter().collect();

  // Is it worth mapping in the other direction to simplify plan calculation?
  // [slot] = day, period, day_idx
  let timeslots: Vec<(&str, usize, usize)> = days
    .iter()
    .map(|(day_id, day)| {
      day
        .stundenmerkmale
        .chars()
        .enumerate()
        .filter_map(|(i, ch)| {
          ['v', 'V']
            .contains(&ch)
            .then_some((&day.kurz[..], i, *day_id))
        })
    })
    .flatten()
    .collect();

  let mut filtered_schedule = Array2::default((classes.len(), timeslots.len()));

  for (pl_index, line) in plan.lehrerstunden().iter().enumerate() {
    let Some(subject_idx) = subjects.iter().position(|s| s.kuerzel == line.fach) else {
      // Skip line if subject is not relevant to query
      continue;
    };

    if excluded_teachers.contains(&line.lehrkraft) {
     continue;
    }

    let period_in_day = plan
      .stunden()
      .iter()
      .enumerate()
      .find_map(|(id, (_, std))| (std.kurz == line.tag_stunde.stunde).then_some(id))
      .expect("Ungültige Stunde");

    let Some(slot) = timeslots
      .iter()
      .position(|(day, p, _)| *p == period_in_day && *day == line.tag_stunde.tag)
    else {
      // Skip if this period is excluded from the plan
      continue;
    };

    let class_idx = classes
      .iter()
      .position(|(_id, c)| c.kuerzel == line.klasse)
      .expect("Eintrag für nicht im Plan vorhandene Klasse");

    filtered_schedule[[class_idx, slot]] = Some((subject_idx, pl_index));
  }

  let weight_sum: f64 = subjects.iter().map(|s| s.gewicht).sum();

  let problem = Problem {
    time_slots: timeslots.len(),
    classes: classes.len(),
    // Normalize weights
    subject_weights: subjects.iter().map(|s| s.gewicht / weight_sum).collect(),
    schedule: filtered_schedule,
  };

  let mut subject_counts = Vec::with_capacity(subjects.len());
  (0..subject_counts.capacity()).for_each(|_| subject_counts.push(Default::default()));

  let mut current = (0..problem.time_slots).map(|_| None).collect();

  let mut cost = f64::INFINITY.clone();
  let mut best_used_classes = usize::MAX.clone();

  let solution = {
    let mut best_solution = None;
    problem.search(
      0,
      &mut current,
      &mut vec![],
      &mut subject_counts,
      &mut best_solution,
      &mut cost,
      &mut best_used_classes,
      &mut 0,
      &timeslots
    );
    best_solution
    // TODO: Proper error handling
  }
  .expect("Keine gültige Lösung gefunden");

  info!(
    "Cost of best solution: {cost} ({} classes)",
    best_used_classes
  );

  finalize(&solution, &timeslots)
}

#[tracing::instrument]
fn finalize(solution: &Solution, timeslots: &Vec<(&str, usize, usize)>) -> Vec<Vec<Option<usize>>>{
  let mut result: Vec<Vec<Option<usize>>> = vec![];
  // Pre-fill seven weekdays
  (0..7).for_each(|_| result.push(vec![]));

  // Yes, I know this objectively sucks. Fix it if you have the time; I know I don't.
  for (i, assignment) in solution.assignments.iter().enumerate() {
    let Some(pl_idx) = assignment else {
      continue;
    };

    let (_day, period, day_id) = timeslots[i];

    // pluh
    while result[day_id - 1].len() <= period {
      result[day_id - 1].push(None);
    }

    result[day_id - 1][period] = Some(*pl_idx);
  }

  result
}

// #[wasm_bindgen(start)]
// fn start() {
//   tracing_subscriber::fmt()
//     .with_writer(tracing_subscriber_wasm::MakeConsoleWriter::default())
//     .without_time()
//     .init();
// }

//// INITIALIZATION ////
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  // print pretty errors in wasm https://github.com/rustwasm/console_error_panic_hook
  // This is not needed for tracing_wasm to work, but it is a common tool for getting proper error line numbers for panics.
  console_error_panic_hook::set_once();

  let mut config = WasmLayerConfig::default();
  config.set_max_level(tracing::metadata::Level::INFO);

  // Add this line:
  wasm_tracing::set_as_global_default_with_config(config).ok();

  Ok(())
}
