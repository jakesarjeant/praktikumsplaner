use dioxus::prelude::*;

use crate::components::{card::Card, input_row::InputRow, button::Button};

#[component]
pub fn StudentForm() -> Element {
  rsx! {
    Card {
      title: rsx! { Fragment { "Praktikanten-Eigenschaften" } },

      InputRow {
        label: "Name des Praktikanten",

        Button {
        }
      }
    }
  }
}
