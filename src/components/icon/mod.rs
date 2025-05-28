use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IconShape(&'static str);

#[derive(PartialEq, Clone, Props)]
pub struct IconProps {
  size: Option<usize>,
  icon: IconShape,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
  rsx! {
    svg {
      xmlns: "http://www.w3.org/2000/svg",
      width: props.size.unwrap_or(24),
      height: props.size.unwrap_or(24),
      view_box: "0 0 256 256",
      preserve_aspect_ratio: "xMidYMid slice",
      dangerous_inner_html: props.icon.0
    }
  }
}

macro_rules! include_icon {
  ($name:tt, $path:literal) => {
    pub const $name: IconShape =
      IconShape(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $path)));
  };
}

// Include Icons
// HACK: In order to be able to easily adjust features of the outer `svg` tag, the svg tag is
// provided by the `Icon` component. The outer `<svg/>` tag _MUST_ be removed from the file.
// HACK: You should set the color in your icon SVG to `currentColor` so that the CSS text color is
// respected.
include_icon!(ARROW_RIGHT, "/assets/phosphor-icons/arrow-right.svg");
include_icon!(FILE_TEXT, "/assets/phosphor-icons/file-text.svg");
include_icon!(FOLDER_DASHED, "/assets/phosphor-icons/folder-dashed.svg");
include_icon!(SPINNER_BALL, "/assets/phosphor-icons/spinner-ball.svg");
include_icon!(CHECK, "/assets/phosphor-icons/check.svg");
include_icon!(X, "/assets/phosphor-icons/x.svg");
