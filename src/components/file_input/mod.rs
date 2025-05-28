use dioxus::prelude::*;

use crate::{
  components::icon::{Icon, CHECK, FILE_TEXT, FOLDER_DASHED, SPINNER_BALL, X},
  hooks::file_upload::FileUpload,
  style,
};

#[derive(PartialEq, Clone, Props)]
pub struct FileInputProps {
  target: FileUpload,
}

#[component]
pub fn FileInput(props: FileInputProps) -> Element {
  style!("/src/components/file_input/file_input.css");

  // HACK: Working around the borrow checker...
  let props2 = props.clone();
  let upload_file = move |evt: FormEvent| {
    let mut target = props2.target.clone();
    async move {
      if let Some(file_engine) = evt.files() {
        let files = file_engine.files();
        let file_name = files.first().expect("Missing file");

        target.begin(file_name.to_string());

        if let Some(content) = file_engine.read_file_to_string(file_name).await {
          target.finish(content);
        } else {
          target.abort();
        }
      }
    }
  };

  rsx! {
    span {
      class: "file-input-wrapper",
      class: if !props.target.is_valid() { "fi-error" },
      span {
        class: "fi-icon-box",
        Icon {
          size: 18,
          icon: if props.target.is_empty() {
            FOLDER_DASHED
          } else {
            FILE_TEXT
          }
        }
      }
      span {
        class: "fi-file-name",
        {props.target.file_name().unwrap_or("Bitte Auswählen".to_string())}
      }
      if !props.target.is_empty() {
        if props.target.is_ready() {
          span {
            class: "fi-icon-box",
            class: if props.target.is_valid() { "fi-success" } else { "fi-error" },
            Icon {
              size: 18,
              icon: if props.target.is_valid() { CHECK } else { X }
            }
          }
        } else {
          span {
            class: "fi-spinner",
            Icon {
              size: 18,
              icon: SPINNER_BALL
            }
          },
        }
      }
      span {
        class: "fi-progress-bar"
      }

      input {
        class: "fi-input",
        type: "file",
        disabled: !props.target.is_ready(),
        oninput: upload_file
      }
    }
  }
}
