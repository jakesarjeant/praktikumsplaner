use dioxus::prelude::*;

use crate::style;

use super::{icon::{Icon, IconShape}, flex::Column};

#[derive(Props, PartialEq, Clone)]
pub struct InputRowProps {
  label: String,
  icon: Option<IconShape>,
  description: Option<Element>,
  children: Element
}

#[component]
pub fn InputRow(props: InputRowProps) -> Element {
  style!("/src/components/input_row/input_row.css");

  rsx! {
    Column {
      style: "width: 100%; max-width: unset",
      label {
        class: "input-row",

        span {
          class: "label-column",

          if let Some(icon) = props.icon {
            Icon {
              size: 18,
              icon
            }
          }
          {props.label}
        }
        {props.children}
      }
      if props.description.is_some() {
        p {
          class: "input-row-description",

          {props.description}
        }
      }
    }
  }
}
