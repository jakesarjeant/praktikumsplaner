use dioxus::prelude::*;
use willi::WilliDocument;

use crate::components::{card::Card, input_row::InputRow, text_input::TextInput, button::Button};

#[derive(Clone, PartialEq, Props)]
pub struct StudentFormProps {
  schedule: Signal<Option<WilliDocument>>,
}

#[component]
pub fn StudentForm(props: StudentFormProps) -> Element {
  rsx! {
    Card {
      title: rsx! { Fragment { "Praktikanten-Eigenschaften" } },
      buttons: rsx! {
        Button {
          disabled: true,
          "Plan erstellen"
        }
      },

      InputRow {
        label: "Name des Praktikanten",

        TextInput {
        }
      }

      InputRow {
        label: "Unterrichtsfächer"
      }

      if let Some(document) = props.schedule.read().clone() {
        h4 {
          "[DEBUG] Verfügbare Fächer"
        }

        ul {
          for subject in document.subjects {
            li {
              {subject.kuerzel}
            }
          }
        }
      }
    }
  }
}
