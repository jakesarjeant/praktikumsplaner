use dioxus::prelude::*;
use style::AppStyles;

mod blocks;
mod components;
mod hooks;
mod pages;
mod style;

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx! {
    AppStyles {}
    document::Stylesheet { href: asset!("/assets/index.css") }

    pages::index::IndexPage {}
  }
}
