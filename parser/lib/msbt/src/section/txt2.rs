use crate::{
  Msbt,
  Encoding,
  error::{Error, Result},
  traits::{CalculatesSize, Updates},
};
use super::Section;

use byteordered::Endian;

use std::{
  borrow::Cow,
  ptr::NonNull,
};

#[derive(Debug)]
pub struct Txt2 {
  pub(crate) msbt: NonNull<Msbt>,
  pub(crate) section: Section,
  pub(crate) string_count: u32,
  pub(crate) raw_strings: Vec<Vec<u8>>,
}

impl Txt2 {
  pub fn msbt(&self) -> &Msbt {
    unsafe { self.msbt.as_ref() }
  }

  pub fn section(&self) -> &Section {
    &self.section
  }

  pub fn string_count(&self) -> u32 {
    self.string_count
  }

  pub fn strings(&self) -> Result<Vec<Cow<str>>> {
    match self.msbt().header.encoding {
      Encoding::Utf16 => {
        self.raw_strings
          .iter()
          .map(|r| r.chunks(2)
            .map(|bs| self.msbt().header.endianness.read_u16(bs).expect("reading from chunk failed"))
            .collect::<Vec<u16>>())
          .map(|s| String::from_utf16(&s).map(Cow::from).map_err(Error::InvalidUtf16))
          .collect()
      },
      Encoding::Utf8 => {
        self.raw_strings
          .iter()
          .map(|r| std::str::from_utf8(r.as_slice()).map(Cow::from).map_err(Error::InvalidBorrowedUtf8))
          .collect()
      }
    }
  }

  pub fn set_strings<I, S>(&mut self, strings: I)
    where I: IntoIterator<Item = S>,
          S: Into<String>,
  {
    match self.msbt().header.encoding {
      Encoding::Utf16 => {
        let mut buf = [0; 2];
        self.raw_strings = strings.into_iter()
          .map(Into::into)
          .map(|s| {
            s.encode_utf16()
              .flat_map(|u| {
                self.msbt().header.endianness.write_u16(&mut buf[..], u).expect("failed to write to array");
                buf.to_vec()
              })
              .collect()
          })
          .collect();
      },
      Encoding::Utf8 => self.raw_strings = strings.into_iter()
        .map(Into::into)
        .map(String::into_bytes)
        .collect(),
    }
  }

  pub fn raw_strings(&self) -> &[Vec<u8>] {
    &self.raw_strings
  }

  // can't implement this even incorrectly until control sequence parsing
  // pub fn set_raw_strings<I, S>(&mut self, strings: I) -> crate::error::Result<()>
  //   where I: IntoIterator<Item = S>,
  //         S: Into<Vec<u8>>,
  // {
  //   let _raw_strings: Vec<Vec<u8>> = strings.into_iter().map(Into::into).collect();
  //   unimplemented!() // FIXME
  // }
}

impl CalculatesSize for Txt2 {
  fn calc_size(&self) -> usize {
    self.section.calc_size()
      + std::mem::size_of_val(&self.string_count)
      + std::mem::size_of::<u32>() * self.raw_strings.len() // offsets
      + self.raw_strings.iter().map(Vec::len).sum::<usize>()
  }
}

impl Updates for Txt2 {
  fn update(&mut self) {
    self.string_count = self.raw_strings.len() as u32;
    let all_str_len = self.raw_strings.iter().map(Vec::len).sum::<usize>();
    let new_size = all_str_len // length of all strings
      + self.string_count as usize * std::mem::size_of::<u32>() // all offsets
      + std::mem::size_of_val(&self.string_count); // length of string count
    self.section.size = new_size as u32;
  }
}
