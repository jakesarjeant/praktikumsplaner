use dioxus::prelude::*;

use crate::style;

use super::icon::{Icon, IconShape};

#[derive(PartialEq, Clone, Props)]
pub struct ButtonProps {
  children: Element,
  disabled: Option<bool>,
  icon_after: Option<IconShape>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
  style!("/src/components/button/button.css");

  rsx! {
    button {
      class: "button",
      disabled: props.disabled.unwrap_or(false),

      {props.children}
      if let Some(icon) = props.icon_after {
        Icon {
          size: 14,
          icon
        }
      }
    }
  }
}
