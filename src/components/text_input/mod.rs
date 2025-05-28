use dioxus::prelude::*;

use crate::style;

use super::icon::{Icon, IconShape};

#[derive(PartialEq, Clone, Props)]
pub struct TextInputProps {
  icon_before: Option<IconShape>,
  placeholder: Option<String>
}

#[component]
pub fn TextInput(props: TextInputProps) -> Element {
  style!("/src/components/text_input/text_input.css");

  rsx! {
    label {
      class: "text-input-wrapper",

      if let Some(icon) = props.icon_before {
        span {
        class: "ti-icon-box",
          Icon {
            size: 18,
            icon
          }
        }
      }
      input {
        class: "fi-input",
        type: "text",
        placeholder: props.placeholder,
      }
    }
  }
}
