use crate::{
  Encoding,
  Msbt,
  traits::{CalculatesSize, Updates},
};
use super::Section;

use std::ptr::NonNull;

#[derive(Debug)]
pub struct Atr1 {
  pub(crate) msbt: NonNull<Msbt>,
  pub(crate) section: Section,
  pub(crate) string_count: u32,
  pub(crate) _unknown_1: u32,
  pub(crate) strings: Vec<String>,
}

impl Atr1 {
  pub fn new_unlinked<I, S>(string_count: u32, _unknown_1: u32, strings: I) -> Self
    where I: IntoIterator<Item = S>,
          S: Into<String>,
  {
    let strings: Vec<String> = strings.into_iter().map(Into::into).collect();
    Atr1 {
      msbt: NonNull::dangling(),
      section: Section::new(*b"ATR1", 0),
      string_count,
      _unknown_1,
      strings,
    }
  }

  pub fn msbt(&self) -> &Msbt {
    unsafe { self.msbt.as_ref() }
  }

  pub fn section(&self) -> &Section {
    &self.section
  }

  pub fn string_count(&self) -> u32 {
    self.string_count
  }

  pub fn unknown_1(&self) -> u32 {
    self._unknown_1
  }

  pub fn strings(&self) -> Vec<&str> {
    self.strings.iter().map(AsRef::as_ref).collect()
  }
}

impl Updates for Atr1 {
  fn update(&mut self) {
    let size = self.calc_size() - self.section.calc_size();
    self.section.size = size as u32;
  }
}

impl CalculatesSize for Atr1 {
  fn calc_size(&self) -> usize {
    let multiplier = match self.msbt().header().encoding() {
      Encoding::Utf8 => 1,
      Encoding::Utf16 => 2,
    };
    self.section.calc_size()
      + std::mem::size_of_val(&self.string_count)
      + std::mem::size_of_val(&self._unknown_1)
      + std::mem::size_of::<u32>() * self.strings.len() // offsets
      + self.strings.iter().map(|x| x.as_bytes().len()).sum::<usize>() * multiplier // strings
  }
}
