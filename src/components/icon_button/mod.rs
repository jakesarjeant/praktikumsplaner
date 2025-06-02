use dioxus::prelude::*;

use crate::style;

use super::{
  button::ButtonFlavor,
  icon::{Icon, IconShape},
};

#[derive(Clone, PartialEq, Props)]
pub struct IconButtonProps {
  flavor: Option<ButtonFlavor>,
  icon: IconShape,
  onclick: Option<EventHandler<MouseEvent>>,
  onkeydown: Option<EventHandler<KeyboardEvent>>,
}

#[component]
pub fn IconButton(props: IconButtonProps) -> Element {
  style!("src/components/icon_button/icon_button.css");

  rsx! {
    button {
      class: "icon-button",
      class: if let Some(ButtonFlavor::Danger) = props.flavor { "danger" },
      onclick: move |evt| if let Some(onclick) = props.onclick { onclick.call(evt) },
      onkeydown: move |evt| if let Some(onkeydown) = props.onkeydown { onkeydown.call(evt) },
      Icon {
        size: 19,
        icon: props.icon
      }
    }
  }
}
