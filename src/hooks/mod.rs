use std::sync::Arc;

pub mod file_upload;

#[derive(Clone)]
pub struct Predicate<A, R> {
  inner: Arc<dyn FnMut(A) -> R>,
}

impl<A, R> PartialEq for Predicate<A, R> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.inner.as_ref(), other.inner.as_ref())
  }
}

impl<A, R, F> From<F> for Predicate<A, R>
where
  F: FnMut(A) -> R + 'static,
{
  fn from(value: F) -> Self {
    Predicate {
      inner: Arc::new(value)
    }
  }
}
