use dioxus::prelude::*;

use crate::{
  blocks::{schedule_form::ScheduleForm, student_form::StudentForm},
  components::{footer::Footer, header::Header, page_container::PageContainer},
};

#[component]
pub fn IndexPage() -> Element {
  rsx! {
    PageContainer {
      Header {}

      ScheduleForm {}
      StudentForm {}

      Footer {}
    }
  }
}
