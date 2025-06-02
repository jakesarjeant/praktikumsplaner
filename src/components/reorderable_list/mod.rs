use dioxus::prelude::*;

use super::{button::ButtonFlavor, icon_button::IconButton};
use crate::{
  components::icon::{DOTS_SIX_VERTICAL, TRASH},
  style,
};

#[derive(Clone, PartialEq, Props)]
pub struct ReorderableListProps<T>
where
  T: Clone + PartialEq + 'static,
{
  items: Signal<Vec<ReorderableListItem<T>>>,
  render: Callback<(String, Signal<Vec<ReorderableListItem<T>>>), Element>,
}

#[derive(Clone, PartialEq)]
pub struct ReorderableListItem<T>
where
  T: Clone + PartialEq + 'static,
{
  pub key: String,
  pub data: T,
}

#[component]
pub fn ReorderableList<T>(mut props: ReorderableListProps<T>) -> Element
where
  T: Clone + PartialEq + 'static,
{
  style!("src/components/reorderable_list/reorderable_list.css");

  let mut active_input = use_signal::<Option<usize>>(|| None);

  to_owned![props.items];

  rsx! {
    div {
      class: "rl-wrapper",
      for (i, item) in props.items.read().iter().enumerate() {
        div {
          key: item.key,

          class: "rl-item",
          class: if Some(i) == active_input.read().clone() { "rl-item-active" },

          IconButton {
            icon: DOTS_SIX_VERTICAL,
            onclick: move |_| {
              if Some(i) == active_input.read().clone() {
                active_input.set(None);
              } else {
                active_input.set(Some(i));
              }
            },
            onkeydown: move |evt: KeyboardEvent| {
              let active_i = active_input.read().clone();
              println!("key pressed");
              if let Some(i) = active_i {
                match &evt.data().key() {
                  Key::ArrowDown if (i + 1) < props.items.len() => {
                    props.items.write().swap(i, i + 1);
                    active_input.set(Some(i + 1));
                  },
                  Key::ArrowUp if i > 0 => {
                    props.items.write().swap(i - 1, i);
                    active_input.set(Some(i - 1));
                  },
                  _ => {}
                }
              }
            },
          }

          span {
            class: "number",
            {(i+1).to_string()} "."
          }

          {props.render.call((item.key.clone(), items))}

          IconButton {
            flavor: ButtonFlavor::Danger,
            icon: TRASH
          }
        }
      }
    }
  }
}
