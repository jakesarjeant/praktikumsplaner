use dioxus::prelude::*;

use crate::style;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
  title: Option<Element>,
  children: Element
}

#[component]
pub fn Card(props: CardProps) -> Element {
  style!("src/components/card/card.css");

  rsx! {
    div {
      class: "card-wrapper",

      if props.title.is_some() {
        h3 {
          class: "card-title",
          {props.title}
        }
      }

      div {
        class: "card",
        {props.children}
      }
    }
  }
}
