use crate::{
  Msbt,
  traits::{CalculatesSize, Updates},
  updater::Updater,
};
use super::Section;

use std::{
  borrow::Cow,
  ptr::NonNull,
};

#[derive(Debug)]
pub struct Lbl1 {
  pub(crate) msbt: NonNull<Msbt>,
  pub(crate) section: Section,
  pub(crate) group_count: u32,
  pub(crate) groups: Vec<Group>,
  pub(crate) labels: Vec<Label>,
}

impl Lbl1 {
  pub fn msbt(&self) -> &Msbt {
    unsafe { self.msbt.as_ref() }
  }

  fn msbt_mut(&mut self) -> Updater<Msbt> {
    Updater::new(unsafe { self.msbt.as_mut() })
  }

  pub fn section(&self) -> &Section {
    &self.section
  }

  pub fn group_count(&self) -> u32 {
    self.group_count
  }

  pub fn groups(&self) -> &[Group] {
    &self.groups
  }

  pub fn labels(&self) -> &[Label] {
    &self.labels
  }

  pub fn labels_mut(&mut self) -> &mut [Label] {
    &mut self.labels
  }

  fn update_group_offsets(&mut self) {
    let mut total = 0;
    let group_len = self.groups.len() as u32;
    for (i, group) in self.groups.iter_mut().enumerate() {
      group.offset = group_len * group.calc_size() as u32
        + std::mem::size_of::<u32>() as u32 // group count
        + total;
      total = self.labels
        .iter()
        .filter(|x| x.checksum == i as u32)
        .fold(total, |current, lbl| current + lbl.calc_size() as u32);
    }
  }
}

#[derive(Debug)]
pub struct Group {
  pub(crate) label_count: u32,
  pub(crate) offset: u32,
}

impl Group {
  pub fn label_count(&self) -> u32 {
    self.label_count
  }

  pub fn offset(&self) -> u32 {
    self.offset
  }
}

#[derive(Debug, Clone)]
pub struct Label {
  pub(crate) lbl1: NonNull<Lbl1>,
  pub(crate) name: String,
  pub(crate) index: u32,
  pub(crate) checksum: u32,
}

impl Label {
  pub(crate) const HASH_MAGIC: u32 = 0x492;

  fn lbl1(&self) -> &Lbl1 {
    unsafe { self.lbl1.as_ref() }
  }

  fn lbl1_mut(&mut self) -> Updater<Lbl1> {
    Updater::new(unsafe { self.lbl1.as_mut() })
  }

  pub(crate) fn update_checksum(&mut self) {
    let hash: u32 = self.name.as_bytes()
      .iter()
      .fold(0, |hash, b| hash.overflowing_mul(Label::HASH_MAGIC).0.overflowing_add(u32::from(*b)).0);
    self.checksum = hash % self.lbl1().group_count();
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn set_name<S>(&mut self, name: S)
    where S: Into<String>,
  {
    self.name = name.into();
    self.update_checksum();
  }

  pub fn index(&self) -> u32 {
    self.index
  }

  pub fn checksum(&self) -> u32 {
    self.checksum
  }

  /// Gets the value of this label.
  ///
  /// Note that the value is not guaranteed to exist. The Msbt containing the Lbl1 of this label
  /// will have its Txt2 checked for this label's index, then that string returned if it exists.
  pub fn value(&self) -> Option<Cow<str>> {
    self.lbl1().msbt().txt2
      .as_ref()
      .and_then(|t| t.strings().ok())
      .and_then(|ss| ss.get(self.index as usize).map(Clone::clone))
  }

  /// Sets the value of this label.
  ///
  /// This checks the Txt2 of the Msbt containing the Lbl1 of this label for this label's index,
  /// then sets that index if it exists.
  pub fn set_value<S: Into<String>>(&mut self, val: S) -> Result<(), ()> {
    let string = val.into();
    self.set_value_raw(string)
  }

  /// Gets the value of this label.
  ///
  /// # Panics
  ///
  /// This method will panic is the Msbt containing this label's Lbl1 does not have a Txt2 or if
  /// that Txt2 does not have a string at this label's index.
  pub unsafe fn value_unchecked(&self) -> Cow<str> {
    self.lbl1().msbt().txt2.as_ref().unwrap().strings().unwrap()[self.index as usize].clone()
  }

  pub fn value_raw(&self) -> Option<&[u8]> {
    self.lbl1().msbt().txt2
      .as_ref()
      .and_then(|t| t.raw_strings.get(self.index as usize).map(AsRef::as_ref))
  }

  pub unsafe fn value_raw_unchecked(&self) -> &[u8] {
    &self.lbl1().msbt().txt2.as_ref().unwrap().raw_strings[self.index as usize]
  }

  /// Sets the raw value of this label.
  ///
  /// This checks the Txt2 of the Msbt containing the Lbl1 of this label for this label's index,
  /// then sets that index if it exists.
  pub fn set_value_raw<S: Into<Vec<u8>>>(&mut self, val: S) -> Result<(), ()> {
    let bytes = val.into();
    let index = self.index as usize;

    let mut lbl1_mut = self.lbl1_mut();
    let mut msbt_mut = lbl1_mut.msbt_mut();
    let txt2 = msbt_mut.txt2.as_mut();

    if let Some(txt2) = txt2 {
      let txt2_raw = txt2.raw_strings.get_mut(index as usize);
      if let Some(txt2_raw) = txt2_raw {
        *txt2_raw = bytes;
        return Ok(());
      }
    }

    Err(())
  }
}

impl Updates for Lbl1 {
  fn update(&mut self) {
    self.section.size = self.calc_size() as u32 - self.section.calc_size() as u32;
    self.update_group_offsets();
    let mut msbt_mut = self.msbt_mut();
    let txt2 = msbt_mut.txt2.as_mut();
    if let Some(txt2) = txt2 {
      txt2.update();
    }
  }
}

impl CalculatesSize for Lbl1 {
  fn calc_size(&self) -> usize {
    self.section.calc_size()
      + std::mem::size_of_val(&self.group_count)
      + self.groups.iter().map(&CalculatesSize::calc_size).sum::<usize>()
      + self.labels.iter().map(&CalculatesSize::calc_size).sum::<usize>()
  }
}

impl CalculatesSize for Group {
  fn calc_size(&self) -> usize {
    std::mem::size_of_val(&self.label_count) + std::mem::size_of_val(&self.offset)
  }
}

impl CalculatesSize for Label {
  fn calc_size(&self) -> usize {
    std::mem::size_of::<u8>() // name length
      + self.name.as_bytes().len()
      + std::mem::size_of_val(&self.index)
  }
}
