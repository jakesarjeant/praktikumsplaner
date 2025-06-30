use ndarray::Array2;
use wasm_bindgen::prelude::*;
use willi::WilliStundenplan;

#[wasm_bindgen]
pub struct Problem {
  time_slots: usize,
  classes: usize,
  // [idx] = (subject, weight)
  /// Weights must add up to 1
  subject_weights: Vec<(usize, f64)>,
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
        .map(|(&count, (_, weight))| {
          let ideal = problem.time_slots as f64 * weight;
          (count as f64 - ideal).abs() / ideal
        })
        .sum::<f64>();

      num_classes + BALANCE_WT * imbalance
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
        let sub_idx = self
          .subject_weights
          .iter()
          .position(|(s, _w)| s == subject)
          .expect("Invalid problem: All subjects included in the schedule should have a weight");
        subject_counts[sub_idx] += 1;

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
        subject_counts[sub_idx] -= 1;
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
impl Problem {
  #[wasm_bindgen(constructor)]
  /// Erstellt ein neues Optimierungsproblem
  ///
  /// # Parameter
  ///
  /// * `plan` — ein WILLI2-Stundenplan
  /// * `subjects` — eine Liste von Fächerkürzeln im Plan, gepaart mit gewichtungen. Kein Fach darf
  /// zweimal vorkommen.
  pub fn new(plan: &WilliStundenplan, subjects: Vec<(String, f64)>) -> Self {
    // NOTE: This assumes each subject only appears once.

    let filtered_schedule = Array2::defaults((plan.klassen().len(), plan.stunden_pro_woche()));
  }
}
