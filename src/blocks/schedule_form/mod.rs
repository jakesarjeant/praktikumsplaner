use dioxus::prelude::*;

use crate::components::card::Card;

#[component]
pub fn ScheduleForm() -> Element {
  rsx! {
    Card {
      title: rsx!{ Fragment { "Stundenplan Auswählen" } },
      note: rsx! { p { "hi" } },

      p {
        "Öffnen sie einen WILLI-Stundenplan oder wählen sie einen bereits importierten aus dem drop-down-menü aus."
      }

      p {
        "Klicken sie auf das Feld, um eine Datei auszusuchen, oder ziehen sie die Datei auf diesen Kasten."
      }
    }
  }
}
