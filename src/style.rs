use std::{
  collections::{BTreeSet, HashSet},
  hash::Hash,
  sync::Mutex,
};

use dioxus::prelude::*;
use once_cell::sync::Lazy;

pub struct HashAsset(pub Asset, pub &'static str);

impl Hash for HashAsset {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.1.hash(state);
  }
}

impl PartialEq for HashAsset {
  fn eq(&self, other: &Self) -> bool {
    self.1.eq(other.1)
  }
}

impl Eq for HashAsset {}

pub static CSS_SOURCES: GlobalSignal<HashSet<HashAsset>> = Global::new(|| HashSet::new());

// TODO: Somehow make it possible to import styles by relative path
#[macro_export]
macro_rules! style {
  ($path:literal) => {{
    crate::style::CSS_SOURCES
      .write()
      .insert(crate::style::HashAsset(asset!($path), $path));
  }};
}

#[component]
pub fn AppStyles() -> Element {
  let sources = CSS_SOURCES.read();
  rsx! {
    Fragment {
      {sources.iter().map(|asset| rsx!{
        document::Stylesheet { href: asset.0  }
      })}
    }
  }
}
