use crate::traits::Updates;

use std::ops::{Deref, DerefMut};

pub struct Updater<'a, T>
  where T: Updates,
{
  inner: &'a mut T,
}

impl<'a, T> Updater<'a, T>
  where T: Updates,
{
  pub(crate) fn new(inner: &'a mut T) -> Self {
    Updater { inner }
  }
}

impl<'a, T> Drop for Updater<'a, T>
  where T: Updates,
{
  fn drop(&mut self) {
    self.inner.update();
  }
}

impl<'a, T> Deref for Updater<'a, T>
  where T: Updates,
{
  type Target = T;

  fn deref(&self) -> &Self::Target {
    self.inner
  }
}

impl<'a, T> DerefMut for Updater<'a, T>
  where T: Updates,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.inner
  }
}
