use dioxus::prelude::*;
use willi::WilliDocument;

use crate::{
  blocks::{schedule_form::ScheduleForm, student_form::StudentForm},
  components::{footer::Footer, header::Header, page_container::PageContainer},
};

#[component]
pub fn IndexPage() -> Element {
  let schedule = use_signal::<Option<WilliDocument>>(|| None);

  rsx! {
    PageContainer {
      Header {}

      ScheduleForm {
        schedule
      }
      StudentForm {
        schedule
      }

      Footer {}
    }
  }
}
