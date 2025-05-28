use dioxus::prelude::*;

use crate::{
  components::icon::{CHECK, FILE_TEXT, FOLDER_DASHED, Icon, SPINNER_BALL, X},
  style,
};

pub struct FileUpload {
  name: String,
  content: String,
}

#[derive(PartialEq, Clone, Props)]
pub struct FileInputProps {
  handle_upload: Callback<FileUpload, Result<(), ()>>,
}

#[component]
pub fn FileInput(props: FileInputProps) -> Element {
  style!("/src/components/file_input/file_input.css");

  let mut upload_state: Signal<UploadState> = use_signal(|| UploadState::Empty);

  let upload_file = move |evt: FormEvent| async move {
    if let Some(file_engine) = evt.files() {
      let files = file_engine.files();
      let file_name = files.first().expect("Missing file");
      upload_state.set(UploadState::InProgress {
        name: file_name.clone(),
      });
      if let Some(content) = file_engine.read_file_to_string(file_name).await {
        let success = props
          .handle_upload
          .call(FileUpload {
            name: file_name.clone(),
            content,
          })
          .is_ok();
        upload_state.set(UploadState::Done {
          name: file_name.clone(),
          success,
        });
      }
    }
  };

  rsx! {
    span {
      class: "file-input-wrapper",
      class: if let UploadState::Done { success: false, .. } = *upload_state.read() { "fi-error" },
      span {
        class: "fi-icon-box",
        Icon {
          size: 18,
          icon: match *upload_state.read() {
            UploadState::Empty => FOLDER_DASHED,
            _ => FILE_TEXT
          }
        }
      }
      span {
        class: "fi-file-name",
        if let UploadState::Done { ref name, success: _ } |
                UploadState::InProgress { ref name } = *upload_state.read() {
          {name.clone()}
        } else {
          "Bitte Auswählen"
        }
      }
      match *upload_state.read() {
        UploadState::InProgress {..} => rsx !{
          span {
            class: "fi-spinner",
            Icon {
              size: 18,
              icon: SPINNER_BALL
            }
          },
        },
        UploadState::Done {success,..} => rsx! {
          span {
            class: "fi-icon-box",
            class: if success { "fi-success" } else { "fi-error" },
            Icon {
              size: 18,
              icon: if success { CHECK } else { X }
            }
          }
        },
        _ => rsx!{}
      }
      span {
        class: "fi-progress-bar"
      }

      input {
        class: "fi-input",
        type: "file",
        oninput: upload_file
      }
    }
  }
}

enum UploadState {
  Empty,
  InProgress { name: String },
  Done { name: String, success: bool },
}
