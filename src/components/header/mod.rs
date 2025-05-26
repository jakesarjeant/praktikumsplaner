use dioxus::prelude::*;

use crate::style;

#[component]
pub fn Header() -> Element {
  style!("/src/components/header/header.css");

  rsx! {
    div {
      class: "header",

      div {
        class: "header-item"
      }
      div {
        class: "header-item header-item-center",

        h1 {
          "Praktikumsplaner"
        }
      }
      div {
        class: "header-item"
      }
    }
  }
}
