use dioxus::prelude::*;

use crate::components::{button::Button, card::Card, flex::{Column, Row}, icon::ARROW_RIGHT};

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

      Row {
        Column {
          flex: "1 1 0",
          p {
            "Öffnen sie einen WILLI-Stundenplan oder wählen sie einen bereits importierten aus dem
            drop-down-menü aus."
          }
          p {
            "Klicken sie auf das Feld, um eine Datei auszusuchen, oder ziehen sie die Datei auf diesen
            Kasten."
          }
        }

        Row{
          flex: "1 1 0",
          p {
            "hi"
          }
        }
      }
      p {
        b {
          "Wichtig: "
        }
        "Es wird eine komplette Stundenplandatei erwartert. Es ist kein manueller Export von Daten
        nötig; ein solcher wird auch nicht akzeptiert."
      }
    }
  }
}
