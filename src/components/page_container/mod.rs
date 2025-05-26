use dioxus::prelude::*;

use crate::style;

#[component]
pub fn PageContainer(children: Element) -> Element {
  style!("/src/components/page_container/page_container.css");

  rsx! {
    div {
      class: "page-container",
      {children}
    }
  }
}
