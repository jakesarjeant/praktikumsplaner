use dioxus::prelude::*;

use crate::components::{card::Card, input_row::InputRow, text_input::TextInput, button::Button};

#[component]
pub fn StudentForm() -> Element {
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
    }
  }
}
