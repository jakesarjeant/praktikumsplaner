use dioxus::prelude::*;

use crate::components::card::Card;

#[component]
pub fn ScheduleForm() -> Element {
  rsx! {
    Card {
      title: rsx!{ Fragment { "Stundenplan Auswählen" } }
    }
  }
}
