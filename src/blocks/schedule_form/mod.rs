use std::sync::Arc;

use dioxus::prelude::*;
use encoding_rs::WINDOWS_1252;
use willi::WilliDocument;

use crate::{
  components::{
    button::Button,
    card::Card,
    file_input::FileInput,
    icon::{ARROW_RIGHT, FILE_TEXT},
    input_row::InputRow,
  },
  hooks::file_upload::{use_file_upload, UploadedFile},
};

#[derive(Clone, PartialEq, Props)]
pub struct ScheduleFormProps {
  schedule: Signal<Option<WilliDocument>>
}

#[component]
pub fn ScheduleForm(props: ScheduleFormProps) -> Element {
  let mut schedule_error = use_signal::<Option<willi::DocumentError>>(|| None);

  let on_upload = use_callback(move |file: Arc<UploadedFile>| {
    to_owned![props.schedule];

    let (content_utf8, _, _) = WINDOWS_1252.decode(&file.content);

    match content_utf8.parse() {
      Ok(doc) => {
        schedule.set(Some(doc));
        schedule_error.set(None);
        Ok(())
      },
      Err(e) => {
        schedule_error.set(Some(e));
        Err(())
      }
    }
  });

  let willi2_file = use_file_upload(|| None, on_upload);

  rsx! {
    Card {
      title: rsx!{ Fragment { "Stundenplan Auswählen" } },
      // buttons: rsx! {
      //   Button {
      //     disabled: !willi2_file.is_valid() || willi2_file.is_empty(),
      //     icon_after: ARROW_RIGHT,
      //     "Weiter"
      //   }
      // },

      p {
        "Öffnen sie eine WILLI2-Datei (Endung \".BAL\"), um mit der Planung loszulegen."
      }
      p {
        "Klicken sie auf das Feld, um eine Datei auszusuchen, oder ziehen sie die Datei auf diesen
        Kasten."
      }
      InputRow {
        label: ".BAL-Datei",
        icon: FILE_TEXT,
        description: rsx! {
          b {
            "Wichtig: "
          }
          "Es wird eine komplette Stundenplandatei erwartert. Es ist kein manueller Export von Daten
          nötig; ein solcher wird auch nicht akzeptiert."
        },
        // TODO: Only allow files with correct ending
        FileInput {
          target: willi2_file
        }
      }
    }
  }
}
