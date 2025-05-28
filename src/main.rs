use dioxus::prelude::*;
use style::AppStyles;

mod style;
mod components;
mod pages;
mod blocks;
mod hooks;

fn main() {
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  rsx!{
    AppStyles {}
    document::Stylesheet { href: asset!("/assets/index.css") }

    pages::index::IndexPage {}
  }
}
