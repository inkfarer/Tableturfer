use crate::{
  Encoding,
  Header,
  Msbt,
  SectionTag,
  section::*,
  traits::Updates,
};

use byteordered::Endianness;

use std::{
  boxed::Box,
  pin::Pin,
  ptr::NonNull,
};

pub struct MsbtBuilder {
  section_order: Vec<SectionTag>,
  header: Header,
  lbl1: Option<Pin<Box<Lbl1>>>,
  txt2: Option<Txt2>,
  nli1: Option<Nli1>,
  ato1: Option<Ato1>,
  atr1: Option<Atr1>,
  tsy1: Option<Tsy1>,
}

  // macro_rules! add_item {
  //   ($lower:ident, $upper:ident) => {
  //     pub fn $lower(mut self, $lower: $upper) -> Self {
  //       self.section_order.remove_item(&SectionTag::$upper);
  //       self.section_order.push(SectionTag::$upper);
  //       self.$lower = Some($lower);

  //       self
  //     }
  //   };
  // }

impl MsbtBuilder {
  pub fn new(endianness: Endianness, encoding: Encoding, group_count: Option<u32>) -> Self {
    let lbl1 = group_count
      .map(|gc| {
        let groups = (0..gc)
          .map(|_| crate::section::lbl1::Group {
            label_count: 0,
            offset: 0,
          })
          .collect();
        Lbl1 {
          msbt: NonNull::dangling(),
          section: Section::new(*b"LBL1", 0),
          group_count: gc,
          groups,
          labels: Vec::with_capacity(gc as usize),
        }
      })
      .map(Box::pin);
    let txt2 = group_count.map(|_| Txt2 {
      msbt: NonNull::dangling(),
      section: Section::new(*b"TXT2", 0),
      string_count: 0,
      raw_strings: Vec::new(),
    });
    let (section_count, section_order) = if group_count.is_some() {
      let mut order = Vec::with_capacity(6);
      order.push(SectionTag::Lbl1);
      order.push(SectionTag::Txt2);
      (2, order)
    } else {
      (0, Vec::with_capacity(6))
    };
    MsbtBuilder {
      section_order,
      header: Header {
        magic: crate::HEADER_MAGIC,
        endianness,
        _unknown_1: 0,
        encoding,
        _unknown_2: 3,
        section_count,
        _unknown_3: 0,
        file_size: 0,
        padding: [0; 10],
      },
      lbl1,
      txt2,
      nli1: None,
      ato1: None,
      atr1: None,
      tsy1: None,
    }
  }

  pub fn header(&self) -> &Header {
    &self.header
  }

  pub fn build(self) -> Pin<Box<Msbt>> {
    let msbt = Msbt {
      header: self.header,
      section_order: self.section_order,
      lbl1: self.lbl1,
      nli1: self.nli1,
      ato1: self.ato1,
      atr1: self.atr1,
      tsy1: self.tsy1,
      txt2: self.txt2,
    };
    let mut pinned_msbt = Box::pin(msbt);

    let msbt_ref: &mut Msbt = unsafe {
      let mut_ref: Pin<&mut Msbt> = Pin::as_mut(&mut pinned_msbt);
      Pin::get_unchecked_mut(mut_ref)
    };
    let ptr = NonNull::new(msbt_ref as *mut Msbt).unwrap();
    if let Some(mut atr1) = msbt_ref.atr1.as_mut() {
      atr1.msbt = ptr;
      atr1.update();
    }
    if let Some(lbl1) = msbt_ref.lbl1.as_mut() {
      lbl1.msbt = ptr;
      lbl1.update();
    }
    if let Some(mut nli1) = msbt_ref.nli1.as_mut() {
      nli1.msbt = ptr;
    }
    if let Some(mut ato1) = msbt_ref.ato1.as_mut() {
      ato1.msbt = ptr;
    }
    if let Some(mut tsy1) = msbt_ref.tsy1.as_mut() {
      tsy1.msbt = ptr;
    }
    if let Some(mut txt2) = msbt_ref.txt2.as_mut() {
      txt2.msbt = ptr;
      txt2.update();
    }

    pinned_msbt.update();

    pinned_msbt
  }

  pub fn add_label<N: Into<String>, V: Into<Vec<u8>>>(mut self, name: N, value: V) -> Self {
    let name = name.into();
    let value = value.into();

    let lbl1 = match self.lbl1.as_mut() {
      Some(l) => l,
      None => return self,
    };
    let txt2 = match self.txt2.as_mut() {
      Some(l) => l,
      None => return self,
    };

    let label = crate::section::lbl1::Label {
      lbl1: {
        let lbl1_ref: &mut Lbl1 = unsafe {
          let mut_ref: Pin<&mut Lbl1> = Pin::as_mut(lbl1);
          Pin::get_unchecked_mut(mut_ref)
        };
        NonNull::new(lbl1_ref as *mut Lbl1).unwrap()
      },
      checksum: name.as_bytes()
        .iter()
        .fold(0u32, |hash, b| hash
          .overflowing_mul(crate::section::lbl1::Label::HASH_MAGIC).0
          .overflowing_add(u32::from(*b)).0) % lbl1.group_count,
      name,
      index: txt2.raw_strings().len() as u32,
    };

    txt2.raw_strings.push(value);
    lbl1.groups[label.checksum as usize].label_count += 1;
    lbl1.labels.push(label);

    self
  }

  // pub fn lbl1(mut self, lbl1: Lbl1) -> Self {
  //   self.section_order.remove_item(&SectionTag::Lbl1);
  //   self.section_order.push(SectionTag::Lbl1);

  //   let mut pinned_lbl1 = Box::pin(lbl1);

  //   let lbl1_ref: &mut Lbl1 = unsafe {
  //     let mut_ref: Pin<&mut Lbl1> = Pin::as_mut(&mut pinned_lbl1);
  //     Pin::get_unchecked_mut(mut_ref)
  //   };
  //   let ptr = NonNull::new(lbl1_ref as *mut Lbl1).unwrap();
  //   for mut label in &mut lbl1_ref.labels {
  //     label.lbl1 = ptr;
  //   }

  //   self.lbl1 = Some(pinned_lbl1);

  //   self
  // }

  pub fn nli1(mut self, nli1: Nli1) -> Self {
    if let Some(pos) = self.section_order.iter().position(|x| x == &SectionTag::Nli1) {
      self.section_order.remove(pos);
    }
    self.section_order.push(SectionTag::Nli1);
    self.nli1 = Some(nli1);

    self
  }

  pub fn ato1(mut self, ato1: Ato1) -> Self {
    if let Some(pos) = self.section_order.iter().position(|x| x == &SectionTag::Ato1) {
      self.section_order.remove(pos);
    }
    self.section_order.push(SectionTag::Ato1);
    self.ato1 = Some(ato1);

    self
  }

  pub fn atr1(mut self, atr1: Atr1) -> Self {
    if let Some(pos) = self.section_order.iter().position(|x| x == &SectionTag::Atr1) {
      self.section_order.remove(pos);
    }
    self.section_order.push(SectionTag::Atr1);
    self.atr1 = Some(atr1);

    self
  }

  pub fn tsy1(mut self, tsy1: Tsy1) -> Self {
    if let Some(pos) = self.section_order.iter().position(|x| x == &SectionTag::Tsy1) {
      self.section_order.remove(pos);
    }
    self.section_order.push(SectionTag::Tsy1);
    self.tsy1 = Some(tsy1);

    self
  }

  pub fn txt2(mut self, txt2: Txt2) -> Self {
    if let Some(pos) = self.section_order.iter().position(|x| x == &SectionTag::Txt2) {
      self.section_order.remove(pos);
    }
    self.section_order.push(SectionTag::Txt2);
    self.txt2 = Some(txt2);

    self
  }
}

    // let mut pinned_lbl1 = Box::pin(lbl1);

    // let lbl1_ref: &mut Lbl1 = unsafe {
    //   let mut_ref: Pin<&mut Lbl1> = Pin::as_mut(&mut pinned_lbl1);
    //   Pin::get_unchecked_mut(mut_ref)
    // };
    // let ptr = NonNull::new(lbl1_ref as *mut Lbl1).unwrap();
    // for mut label in &mut lbl1_ref.labels {
    //   label.lbl1 = ptr;
    // }
