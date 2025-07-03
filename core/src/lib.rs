use ndarray::Array2;
use wasm_bindgen::prelude::*;
use willi::WilliStundenplan;

#[wasm_bindgen]
pub struct Problem {
  time_slots: usize,
  classes: usize,
  // [subject] = weight
  /// Weights must add up to 1
  subject_weights: Vec<f64>,
  // [class][slot] = Option<subject>
  schedule: Array2<Option<usize>>,
}

pub struct Solution {
  // [slot] = Option<subject>
  assignments: Vec<Option<usize>>,
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
  ) {
    // Gewichtung der gleichmäßigen verteilung der fächer. 0 = Verteilung wird ignoriert
    const BALANCE_WT: f64 = 0.1;

    // Kostenfunktion
    fn cost(
      problem: &Problem,
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
          let ideal = problem.time_slots as f64 * weight;
          (count as f64 - ideal).abs() / ideal
        })
        .sum::<f64>();

      num_classes + (BALANCE_WT * imbalance)
    }

    // Abbruchbedingung: letzte Stunde erreicht.
    if slot == self.time_slots {
      // Falls die Lösung eine Verbesserung darstellt: Speichern der neuen Lösung
      let current_cost = cost(self, used_classes, subject_counts);
      if current_cost < *best_cost {
        *best = Some(Solution {
          assignments: current.clone(),
        });
        *best_cost = current_cost;
      }
      // Rekursionsabbruch
      return;
    }

    let mut assigned = false;

    for class in 0..self.classes {
      // Falls diese Klasse zu dieser Zeit in einem Fach des Praktikanten unterrichtet wird
      if let Some(subject) = &self.schedule[[class, slot]] {
        // Klasse für diese Stunde eintragen
        current[slot] = Some(class);
        let was_new = if used_classes.contains(&class) {
          false
        } else {
          used_classes.push(class);
          true
        };

        // Verteilungen aktualisieren
        subject_counts[*subject] += 1;

        // Nur weiter suchen, wenn diese Lösung nicht schon schlechter ist als die Letzte
        if cost(self, used_classes, subject_counts) < *best_cost {
          // Weiter bei der nächsten Stunde
          self.search(
            slot + 1,
            current,
            used_classes,
            subject_counts,
            best,
            best_cost,
          )
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
      );
    }
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
pub fn wasm_generate(plan: &WilliStundenplan, subjects: Vec<FachGewichtung>) -> JsValue {
  let solution = generate(plan, &subjects);
  serde_wasm_bindgen::to_value(&solution).unwrap()
}

/// Erstellt einen Stundenplan
///
/// # Parameter
///
/// * `plan` — ein WILLI2-Stundenplan
/// * `subjects` — eine Liste von Fächerkürzeln im Plan, gepaart mit gewichtungen. Kein Fach darf
/// zweimal vorkommen.
pub fn generate(
  plan: &WilliStundenplan,
  subjects: &Vec<FachGewichtung>,
) -> Vec<Vec<Option<String>>> {
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

  for line in plan.lehrerstunden() {
    let Some(subject_idx) = subjects.iter().position(|s| s.kuerzel == line.fach) else {
      // Skip line if subject is not relevant to query
      continue;
    };

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

    filtered_schedule[[class_idx, slot]] = Some(subject_idx)
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

  let solution = {
    let mut best_solution = None;
    problem.search(
      0,
      &mut current,
      &mut vec![],
      &mut subject_counts,
      &mut best_solution,
      &mut cost,
    );
    best_solution
    // TODO: Proper error handling
  }
  .expect("Keine gültige Lösung gefunden");

  println!("best cost: {cost}");

  let mut result: Vec<Vec<Option<String>>> = vec![];
  // Pre-fill seven weekdays
  (0..7).for_each(|_| result.push(vec![]));

  // Yes, I know this objectively sucks. Fix it if you have the time; I know I don't.
  for (i, assignment) in solution.assignments.iter().enumerate() {
    let Some(class_id) = assignment else {
      continue;
    };

    let class_name = classes[*class_id].1.kuerzel.clone();

    let (_day, period, day_id) = timeslots[i];

    // pluh
    while result[day_id - 1].len() <= period {
      result[day_id - 1].push(None);
    }

    result[day_id - 1][period] = Some(class_name);
  }

  result
}
