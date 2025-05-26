use dioxus::prelude::*;

use crate::components::card::Card;

#[component]
pub fn StudentForm() -> Element {
  rsx! {
    Card {
      title: rsx! { Fragment { "Praktikanten-Eigenschaften" } }
    }
  }
}
