use dioxus::prelude::*;

use crate::components::{button::Button, card::Card, file_input::FileInput, icon::{ARROW_RIGHT, FILE_TEXT}, input_row::InputRow};

#[component]
pub fn ScheduleForm() -> Element {
  rsx! {
    Card {
      title: rsx!{ Fragment { "Stundenplan Auswählen" } },
      buttons: rsx! {
        Button {
          disabled: true,
          icon_after: ARROW_RIGHT,
          "Weiter"
        }
      },

      p {
        "Öffnen sie einen WILLI-Stundenplan oder wählen sie einen bereits importierten aus dem
        drop-down-menü aus."
      }
      p {
        "Klicken sie auf das Feld, um eine Datei auszusuchen, oder ziehen sie die Datei auf diesen
        Kasten."
      }
      InputRow {
        label: "WILLI2-Datei",
        icon: FILE_TEXT,
        description: rsx! {
          b {
            "Wichtig: "
          }
          "Es wird eine komplette Stundenplandatei erwartert. Es ist kein manueller Export von Daten
          nötig; ein solcher wird auch nicht akzeptiert."
        },
        // TODO: Only allow files with correct ending
        FileInput {
          handle_upload: move |_| { Err(()) }
        }
      }
    }
  }
}
