use std::ops::DerefMut;

pub(crate) trait CalculatesSize {
  /// Calculate the size of this object when written to an MSBT.
  fn calc_size(&self) -> usize;
}

pub trait Updates {
  /// Update this object with any new changes made.
  fn update(&mut self);
}

impl<T> Updates for std::pin::Pin<T>
  where T: DerefMut,
        T::Target: Unpin + Updates,
{
  fn update(&mut self) {
    <T::Target as Updates>::update(&mut **self)
  }
}
