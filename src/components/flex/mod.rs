use std::fmt::Write;

use dioxus::prelude::*;

use crate::style;

#[derive(PartialEq, Clone, Props)]
pub struct FlexProps {
  #[props(default = true)]
  wrap: bool,
  shrink: Option<usize>,
  flex: Option<String>,
  gap: Option<String>,
  style: Option<String>,
  children: Element,
}

#[component]
pub fn Row(props: FlexProps) -> Element {
  style!("/src/components/flex/flex.css");

  rsx! {
    div {
      class: "flex-row",
      style: {
        let mut style = String::new();
        if let Some(shrink) = props.shrink { write!(&mut style, "flex-shrink: {shrink};").ok(); }
        if let Some(flex) = props.flex { write!(&mut style, "flex: {flex};").ok(); }
        if let Some(gap) = props.gap { write!(&mut style, "gap: {gap};").ok(); }
        if let Some(raw_style) = props.style { write!(&mut style, "{raw_style};").ok(); }
        style
      },
      {props.children}
    }
  }
}

#[component]
pub fn Column(props: FlexProps) -> Element {
  style!("/src/components/flex/flex.css");

  rsx! {
    div {
      class: "flex-column",
      style: {
        let mut style = String::new();
        if let Some(shrink) = props.shrink { write!(&mut style, "flex-shrink: {shrink};").ok(); }
        if let Some(flex) = props.flex { write!(&mut style, "flex: {flex};").ok(); }
        if let Some(gap) = props.gap { write!(&mut style, "gap: {gap};").ok(); }
        if let Some(raw_style) = props.style { write!(&mut style, "{raw_style};").ok(); }
        style
      },
      {props.children}
    }
  }
}
