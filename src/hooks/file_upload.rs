use std::sync::Arc;

use dioxus::prelude::*;

/// Creates a source of truth for a file upload.
///
/// Returns a state object that can be passed to `FileInput` or similar components.
///
/// # Parameters
///
/// * `init` - Initialize the contained file. If an uploaded file is already cached somewhere in
///   the browser, this can be used to load it as a placeholder.
/// * `on_change` - Called every time the file in the upload changes. If it returns `Ok`, the
///   uploaded file is considered validated; If it returns `Err`, the uploaded file is considered
///   invalid.
pub fn use_file_upload<I>(
  init: I,
  on_change: Callback<Arc<UploadedFile>, Result<(), ()>>,
) -> FileUpload
where
  I: FnOnce() -> Option<UploadedFile>,
{
  let is_valid = use_signal(|| true);
  let state = use_signal(|| match init() {
    Some(file) => FileUploadState::Done(Arc::new(file)),
    None => FileUploadState::Empty,
  });

  FileUpload {
    is_valid,
    state,
    on_change: on_change,
  }
}

#[derive(Clone, PartialEq)]
pub struct FileUpload {
  is_valid: Signal<bool>,
  state: Signal<FileUploadState>,
  on_change: Callback<Arc<UploadedFile>, Result<(), ()>>,
}

impl FileUpload {
  pub fn is_valid(&self) -> bool {
    *self.is_valid.read()
  }

  pub fn is_empty(&self) -> bool {
    self.state.read().is_empty()
  }

  pub fn file_name(&self) -> Option<String> {
    match *self.state.read() {
      FileUploadState::Failed { ref file_name } => Some(file_name.clone()),
      FileUploadState::Uploading { ref file_name } => Some(file_name.clone()),
      FileUploadState::Done(ref f) => Some(f.file_name.clone()),
      _ => None
    }
  }

  pub fn file(&self) -> Option<Arc<UploadedFile>> {
    self.state.read().file()
  }

  /// Can we start an upload right now? When this returns `false`, the corresponding input should be
  /// disabled.
  pub fn is_ready(&self) -> bool {
    self.state.read().is_ready()
  }

  /// Start the upload. If `is_ready()` does not return true when this method is called, it will
  /// return `false` and do nothing. After this method is called, you must call `finish()` or
  /// `abort` to return it to the ready state.
  pub fn begin(&mut self, file_name: String) -> bool {
    if self.is_ready() {
      // TODO: Should `true` really be the default state?
      self.is_valid.set(true);
      self.state.set(FileUploadState::Uploading { file_name });

      true
    } else {
      false
    }
  }

  /// Indicate that the upload has failed to complete. Ignored if no upload is in progress.
  pub fn abort(&mut self) {
    // TODO: Maybe avoid cloning the whole file if big files ever beome an issue?
    let FileUploadState::Uploading { file_name } = (*self.state.read()).clone() else {
      return;
    };

    self.state.set(FileUploadState::Failed { file_name })
  }

  pub fn finish(&mut self, content: String) {
    let FileUploadState::Uploading { file_name } = (*self.state.read()).clone() else {
      return;
    };

    let file = Arc::new(UploadedFile { file_name, content });
    match (self.on_change)(file.clone()) {
      Ok(_) => self.is_valid.set(true),
      Err(_) => self.is_valid.set(false),
    }
    self.state.set(FileUploadState::Done(file))
  }
}

#[derive(Clone)]
pub enum FileUploadState {
  Empty,
  Uploading { file_name: String },
  Done(Arc<UploadedFile>),
  Failed { file_name: String },
}

impl FileUploadState {
  pub fn is_empty(&self) -> bool {
    matches!(self, FileUploadState::Empty)
  }

  /// Can we start an upload right now? When this returns `false`, the corresponding input should be
  /// disabled.
  pub fn is_ready(&self) -> bool {
    !matches!(self, FileUploadState::Uploading { .. })
  }

  pub fn file(&self) -> Option<Arc<UploadedFile>> {
    if let FileUploadState::Done(file) = self {
      Some(file.clone())
    } else {
      None
    }
  }
}

#[derive(Clone)]
pub struct UploadedFile {
  file_name: String,
  content: String,
}
