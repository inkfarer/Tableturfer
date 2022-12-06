use std::{
  boxed::Box,
  collections::BTreeMap,
  io::{Read, Seek, SeekFrom, Write},
  pin::Pin,
  ptr::NonNull,
};

use byteordered::{Endianness, Endian};

mod counter;
mod traits;
pub mod builder;
pub mod error;
pub mod section;
pub mod updater;

use self::{
  counter::Counter,
  error::{Error, Result},
  section::{
    *,
    lbl1::{Group, Label},
  },
  traits::{CalculatesSize, Updates},
  updater::Updater,
};

const HEADER_MAGIC: [u8; 8] = *b"MsgStdBn";
// const LABEL_HASH_MAGIC: u16 = 0x492;
// const LABEL_MAX_LEN: u8 = 64;
// const BYTE_ORDER_OFFSET: u8 = 0x8;
// const HEADER_SIZE: u8 = 0x20;
const PADDING_CHAR: u8 = 0xAB;
const PADDING_LENGTH: usize = 16;

#[derive(Debug, PartialEq)]
pub enum SectionTag {
  Lbl1,
  Nli1,
  Ato1,
  Atr1,
  Tsy1,
  Txt2,
}

#[derive(Debug)]
pub struct Msbt {
  pub(crate) header: Header,
  pub(crate) section_order: Vec<SectionTag>,
  // pinned because child labels have a reference to lbl1
  pub(crate) lbl1: Option<Pin<Box<Lbl1>>>,
  pub(crate) nli1: Option<Nli1>,
  pub(crate) ato1: Option<Ato1>,
  pub(crate) atr1: Option<Atr1>,
  pub(crate) tsy1: Option<Tsy1>,
  pub(crate) txt2: Option<Txt2>,
}

impl Msbt {
  pub fn from_reader<R: Read + Seek>(reader: R) -> Result<Pin<Box<Self>>> {
    MsbtReader::new(reader).map(MsbtReader::into_msbt)
  }

  pub fn write_to<W: Write>(&self, writer: W) -> Result<()> {
    let mut writer = MsbtWriter::new(self, writer);
    writer.write_header()?;
    for tag in &self.section_order {
      match *tag {
        SectionTag::Lbl1 => writer.write_lbl1()?,
        SectionTag::Nli1 => writer.write_nli1()?,
        SectionTag::Ato1 => writer.write_ato1()?,
        SectionTag::Atr1 => writer.write_atr1()?,
        SectionTag::Tsy1 => writer.write_tsy1()?,
        SectionTag::Txt2 => writer.write_txt2()?,
      }
    }
    Ok(())
  }

  pub fn header(&self) -> &Header {
    &self.header
  }

  pub fn section_order(&self) -> &[SectionTag] {
    &self.section_order
  }

  pub fn lbl1(&self) -> Option<&Pin<Box<Lbl1>>> {
    self.lbl1.as_ref()
  }

  pub fn lbl1_mut(&mut self) -> Option<Updater<Pin<Box<Lbl1>>>> {
    self.lbl1.as_mut().map(Updater::new)
  }

  pub fn nli1(&self) -> Option<&Nli1> {
    self.nli1.as_ref()
  }

  pub fn nli1_mut(&mut self) -> Option<&mut Nli1> {
    self.nli1.as_mut()
  }

  pub fn ato1(&self) -> Option<&Ato1> {
    self.ato1.as_ref()
  }

  pub fn ato1_mut(&mut self) -> Option<&mut Ato1> {
    self.ato1.as_mut()
  }

  pub fn atr1(&self) -> Option<&Atr1> {
    self.atr1.as_ref()
  }

  pub fn atr1_mut(&mut self) -> Option<&mut Atr1> {
    self.atr1.as_mut()
  }

  pub fn tsy1(&self) -> Option<&Tsy1> {
    self.tsy1.as_ref()
  }

  pub fn tsy1_mut(&mut self) -> Option<&mut Tsy1> {
    self.tsy1.as_mut()
  }

  pub fn txt2(&self) -> Option<&Txt2> {
    self.txt2.as_ref()
  }

  pub fn txt2_mut(&mut self) -> Option<Updater<Txt2>> {
    self.txt2.as_mut().map(Updater::new)
  }

  fn plus_padding(size: usize) -> usize {
    let rem = size % 16;
    if rem > 0 {
      size + (16 - rem)
    } else {
      size
    }
  }
}

impl CalculatesSize for Msbt {
  // can't detect that Lbl1 is a Pin and has to be called in a redundant closure
  #[allow(clippy::redundant_closure)]
  fn calc_size(&self) -> usize {
    self.header.calc_file_size()
      + Msbt::plus_padding(self.lbl1.as_ref().map(|x| x.calc_size()).unwrap_or(0))
      + Msbt::plus_padding(self.nli1.as_ref().map(CalculatesSize::calc_size).unwrap_or(0))
      + Msbt::plus_padding(self.ato1.as_ref().map(CalculatesSize::calc_size).unwrap_or(0))
      + Msbt::plus_padding(self.atr1.as_ref().map(CalculatesSize::calc_size).unwrap_or(0))
      + Msbt::plus_padding(self.tsy1.as_ref().map(CalculatesSize::calc_size).unwrap_or(0))
      + Msbt::plus_padding(self.txt2.as_ref().map(CalculatesSize::calc_size).unwrap_or(0))
  }
}

impl Updates for Msbt {
  fn update(&mut self) {
    self.header.file_size = self.calc_size() as u32;
    self.header.section_count = self.section_order.len() as u16;
  }
}

#[derive(Debug)]
pub struct MsbtWriter<'a, W> {
  writer: Counter<W>,
  msbt: &'a Msbt,
}

impl<'a, W: Write> MsbtWriter<'a, W> {
  fn new(msbt: &'a Msbt, writer: W) -> Self {
    MsbtWriter {
      msbt,
      writer: Counter::new(writer),
    }
  }

  fn write_header(&mut self) -> Result<()> {
    self.writer.write_all(&self.msbt.header.magic).map_err(Error::Io)?;
    let endianness = match self.msbt.header.endianness {
      Endianness::Big => [0xFE, 0xFF],
      Endianness::Little => [0xFF, 0xFE],
    };
    self.writer.write_all(&endianness).map_err(Error::Io)?;
    self.msbt.header.endianness.write_u16(&mut self.writer, self.msbt.header._unknown_1).map_err(Error::Io)?;
    let encoding_byte = match self.msbt.header.encoding {
      Encoding::Utf8 => 0x00,
      Encoding::Utf16 => 0x01,
    };
    self.writer.write_all(&[encoding_byte, self.msbt.header._unknown_2]).map_err(Error::Io)?;
    self.msbt.header.endianness.write_u16(&mut self.writer, self.msbt.header.section_count).map_err(Error::Io)?;
    self.msbt.header.endianness.write_u16(&mut self.writer, self.msbt.header._unknown_3).map_err(Error::Io)?;
    self.msbt.header.endianness.write_u32(&mut self.writer, self.msbt.calc_size() as u32).map_err(Error::Io)?;
    // FIXME: update this as changes are made
    // self.msbt.header.endianness.write_u32(&mut self.writer, self.msbt.header.file_size).map_err(Error::Io)?;
    self.writer.write_all(&self.msbt.header.padding).map_err(Error::Io)
  }

  fn write_section(&mut self, section: &Section) -> Result<()> {
    self.writer.write_all(&section.magic).map_err(Error::Io)?;
    self.msbt.header.endianness.write_u32(&mut self.writer, section.size).map_err(Error::Io)?;
    self.writer.write_all(&section.padding).map_err(Error::Io)
  }

  fn write_group(&mut self, group: &Group) -> Result<()> {
    self.msbt.header.endianness.write_u32(&mut self.writer, group.label_count).map_err(Error::Io)?;
    self.msbt.header.endianness.write_u32(&mut self.writer, group.offset).map_err(Error::Io)
  }

  fn write_lbl1(&mut self) -> Result<()> {
    if let Some(ref lbl1) = self.msbt.lbl1 {
      self.write_section(&lbl1.section)?;
      self.msbt.header.endianness.write_u32(&mut self.writer, lbl1.group_count).map_err(Error::Io)?;
      for group in &lbl1.groups {
        self.write_group(group)?;
      }
      let mut sorted_labels = lbl1.labels.clone(); // FIXME: don't clone
      sorted_labels.sort_by_key(|l| l.checksum);
      for label in &sorted_labels {
        self.writer.write_all(&[label.name.len() as u8]).map_err(Error::Io)?;
        self.writer.write_all(label.name.as_bytes()).map_err(Error::Io)?;
        self.msbt.header.endianness.write_u32(&mut self.writer, label.index).map_err(Error::Io)?;
      }

      self.write_padding()?;
    }
    Ok(())
  }

  pub fn write_nli1(&mut self) -> Result<()> {
    if let Some(ref nli1) = self.msbt.nli1 {
      self.write_section(&nli1.section)?;

      if nli1.section.size > 0 {
        self.msbt.header.endianness.write_u32(&mut self.writer, nli1.id_count).map_err(Error::Io)?;

        for (&key, &val) in &nli1.global_ids {
          self.msbt.header.endianness.write_u32(&mut self.writer, val).map_err(Error::Io)?;
          self.msbt.header.endianness.write_u32(&mut self.writer, key).map_err(Error::Io)?;
        }
      }

      self.write_padding()?;
    }

    Ok(())
  }

  pub fn write_txt2(&mut self) -> Result<()> {
    if let Some(ref txt2) = self.msbt.txt2 {
      self.write_section(&txt2.section)?;

      // write string count
      self.msbt.header.endianness.write_u32(&mut self.writer, txt2.string_count).map_err(Error::Io)?;

      // write offsets
      let mut total = 0;
      for s in &txt2.raw_strings {
        let offset = txt2.string_count * 4 + 4 + total;
        total += s.len() as u32;
        self.msbt.header.endianness.write_u32(&mut self.writer, offset).map_err(Error::Io)?;
      }

      // write strings
      for s in &txt2.raw_strings {
        self.writer.write_all(&s).map_err(Error::Io)?;
      }

      self.write_padding()?;
    }

    Ok(())
  }

  pub fn write_ato1(&mut self) -> Result<()> {
    if let Some(ref ato1) = self.msbt.ato1 {
      self.write_section(&ato1.section)?;
      self.writer.write_all(&ato1._unknown).map_err(Error::Io)?;

      self.write_padding()?;
    }

    Ok(())
  }

  pub fn write_atr1(&mut self) -> Result<()> {
    if let Some(ref atr1) = self.msbt.atr1 {
      self.write_section(&atr1.section)?;
      self.msbt.header.endianness.write_u32(&mut self.writer, atr1.string_count).map_err(Error::Io)?;
      self.msbt.header.endianness.write_u32(&mut self.writer, atr1._unknown_1).map_err(Error::Io)?;

      let raw_strings: Vec<Vec<u8>> = atr1.strings
        .iter()
        .map(|string| match self.msbt.header.encoding() {
          Encoding::Utf16 => {
            let mut buf = [0; 2];
            string.encode_utf16()
              .flat_map(|u| {
                self.msbt.header.endianness.write_u16(&mut buf[..], u).expect("failed to write to array");
                buf.to_vec()
              })
              .collect()
          },
          Encoding::Utf8 => string.as_bytes().to_vec(),
        })
        .collect();

      let mut offset = std::mem::size_of_val(&atr1.string_count)
        + std::mem::size_of_val(&atr1._unknown_1)
        + std::mem::size_of::<u32>() * atr1.strings.len();
      for raw_string in &raw_strings {
        self.msbt.header.endianness.write_u32(&mut self.writer, offset as u32).map_err(Error::Io)?;
        offset += raw_string.len();
      }

      for raw_string in raw_strings {
        self.writer.write_all(&raw_string).map_err(Error::Io)?;
      }

      self.write_padding()?;
    }

    Ok(())
  }

  pub fn write_tsy1(&mut self) -> Result<()> {
    if let Some(ref tsy1) = self.msbt.tsy1 {
      self.write_section(&tsy1.section)?;
      self.writer.write_all(&tsy1._unknown).map_err(Error::Io)?;

      self.write_padding()?;
    }

    Ok(())
  }

  fn write_padding(&mut self) -> Result<()> {
    let remainder = self.writer.written() % PADDING_LENGTH;
    if remainder == 0 {
      return Ok(());
    }

    self.writer.write_all(&vec![PADDING_CHAR; PADDING_LENGTH - remainder]).map_err(Error::Io)
  }
}

#[derive(Debug)]
pub struct MsbtReader<R> {
  reader: R,
  section_order: Vec<SectionTag>,
  header: Header,
  lbl1: Option<Pin<Box<Lbl1>>>,
  nli1: Option<Nli1>,
  ato1: Option<Ato1>,
  atr1: Option<Atr1>,
  tsy1: Option<Tsy1>,
  txt2: Option<Txt2>,
}

impl<R: Read + Seek> MsbtReader<R> {
  fn new(mut reader: R) -> Result<Self> {
    let header = Header::from_reader(&mut reader)?;

    let mut msbt = MsbtReader {
      reader,
      header,
      lbl1: None,
      nli1: None,
      ato1: None,
      atr1: None,
      tsy1: None,
      txt2: None,
      section_order: Vec::with_capacity(6),
    };

    msbt.read_sections()?;

    Ok(msbt)
  }

  fn into_msbt(self) -> Pin<Box<Msbt>> {
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
    if let Some(lbl1) = msbt_ref.lbl1.as_mut() {
      lbl1.msbt = ptr;
    }
    if let Some(mut nli1) = msbt_ref.nli1.as_mut() {
      nli1.msbt = ptr;
    }
    if let Some(mut ato1) = msbt_ref.ato1.as_mut() {
      ato1.msbt = ptr;
    }
    if let Some(mut atr1) = msbt_ref.atr1.as_mut() {
      atr1.msbt = ptr;
    }
    if let Some(mut tsy1) = msbt_ref.tsy1.as_mut() {
      tsy1.msbt = ptr;
    }
    if let Some(mut txt2) = msbt_ref.txt2.as_mut() {
      txt2.msbt = ptr;
    }

    pinned_msbt
  }

  fn skip_padding(&mut self) -> Result<()> {
    let mut buf = [0; 16];
    loop {
      let read = self.reader.read(&mut buf).map_err(Error::Io)?;
      if read == 0 {
        return Ok(());
      }
      if let Some(i) = buf[..read].iter().position(|&x| x != PADDING_CHAR) {
        self.reader.seek(SeekFrom::Current(i as i64 - 16)).map_err(Error::Io)?;
        return Ok(());
      }
    }
  }

  pub fn read_sections(&mut self) -> Result<()> {
    let mut peek = [0; 4];
    loop {
      match self.reader.read_exact(&mut peek) {
        Ok(()) => {},
        Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(()),
        Err(e) => return Err(Error::Io(e)),
      }

      self.reader.seek(SeekFrom::Current(-4)).map_err(Error::Io)?;

      match &peek {
        b"LBL1" => {
          self.lbl1 = Some(self.read_lbl1()?);
          self.section_order.push(SectionTag::Lbl1);
        },
        b"ATR1" => {
          // inky: disable ATR1 parsing as it breaks with our input files and isn't of any use to us
          // self.atr1 = Some(self.read_atr1()?);
          let section = self.read_section()?;
          self.reader.seek(SeekFrom::Current(section.size as i64)).map_err(Error::Io)?;
          self.section_order.push(SectionTag::Atr1);
        },
        b"ATO1" => {
          self.ato1 = Some(self.read_ato1()?);
          self.section_order.push(SectionTag::Ato1);
        },
        b"TSY1" => {
          self.tsy1 = Some(self.read_tsy1()?);
          self.section_order.push(SectionTag::Tsy1);
        },
        b"TXT2" => {
          self.txt2 = Some(self.read_txt2()?);
          self.section_order.push(SectionTag::Txt2);
        },
        b"NLI1" => {
          self.nli1 = Some(self.read_nli1()?);
          self.section_order.push(SectionTag::Nli1);
        },
        _ => return Err(Error::InvalidSection(peek)),
      }

      self.skip_padding()?;
    }
  }

  pub fn read_lbl1(&mut self) -> Result<Pin<Box<Lbl1>>> {
    let section = self.read_section()?;

    if &section.magic != b"LBL1" {
      return Err(Error::InvalidMagic);
    }

    let group_count = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;

    let mut groups = Vec::with_capacity(group_count as usize);

    for _ in 0..group_count {
      groups.push(self.read_group()?);
    }

    let mut labels = Vec::with_capacity(groups.iter().map(|x| x.label_count as usize).sum());

    let mut buf = [0; 1];
    for (i, group) in groups.iter().enumerate() {
      for _ in 0..group.label_count {
        self.reader.read_exact(&mut buf).map_err(Error::Io)?;
        let str_len = buf[0] as usize;

        let mut str_buf = vec![0; str_len];
        self.reader.read_exact(&mut str_buf).map_err(Error::Io)?;
        let name = String::from_utf8(str_buf).map_err(Error::InvalidUtf8)?;
        let index = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;
        let checksum = i as u32;

        labels.push(Label {
          lbl1: NonNull::dangling(),
          name,
          index,
          checksum,
        });
      }
    }

    let lbl1 = Lbl1 {
      msbt: NonNull::dangling(),
      section,
      group_count,
      groups,
      labels,
    };
    let mut pinned_lbl1 = Box::pin(lbl1);

    let lbl1_ref: &mut Lbl1 = unsafe {
      let mut_ref: Pin<&mut Lbl1> = Pin::as_mut(&mut pinned_lbl1);
      Pin::get_unchecked_mut(mut_ref)
    };
    let ptr = NonNull::new(lbl1_ref as *mut Lbl1).unwrap();
    for mut label in &mut lbl1_ref.labels {
      label.lbl1 = ptr;
    }

    Ok(pinned_lbl1)
  }

  pub fn read_atr1(&mut self) -> Result<Atr1> {
    let section = self.read_section()?;

    let string_count = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;
    let _unknown_1 = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;

    self.reader.seek(SeekFrom::Current(4)).map_err(Error::Io)?;

    let strings = if section.size == 8 {
      Vec::new()
    } else {
      let mut offsets = Vec::with_capacity(string_count as usize);
      for _ in 0..string_count {
        offsets.push(self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?);
      }

      let mut strings = Vec::with_capacity(string_count as usize);
      for i in 0..string_count {
        let next_str_end = if i == string_count - 1 {
          section.size
        } else {
          offsets[i as usize + 1]
        };
        let str_len = next_str_end - offsets[i as usize];
        let mut str_buf = vec![0; str_len as usize];
        self.reader.read_exact(&mut str_buf).map_err(Error::Io)?;
        let string = match self.header.encoding {
          Encoding::Utf16 => {
            let u16s: Vec<u16> = str_buf.chunks(2)
              .map(|bs| self.header.endianness.read_u16(bs).expect("reading from chunk failed"))
              .collect();
            String::from_utf16(&u16s).map_err(Error::InvalidUtf16)?
          },
          Encoding::Utf8 => String::from_utf8(str_buf).map_err(Error::InvalidUtf8)?,
        };
        strings.push(string);
      }

      strings
    };

    Ok(Atr1 {
      msbt: NonNull::dangling(),
      section,
      string_count,
      _unknown_1,
      strings,
    })
  }

  pub fn read_ato1(&mut self) -> Result<Ato1> {
    let section = self.read_section()?;
    let mut unknown = vec![0; section.size as usize];
    self.reader.read_exact(&mut unknown).map_err(Error::Io)?;

    Ok(Ato1 {
      msbt: NonNull::dangling(),
      section,
      _unknown: unknown,
    })
  }

  pub fn read_tsy1(&mut self) -> Result<Tsy1> {
    let section = self.read_section()?;
    let mut unknown = vec![0; section.size as usize];
    self.reader.read_exact(&mut unknown).map_err(Error::Io)?;

    Ok(Tsy1 {
      msbt: NonNull::dangling(),
      section,
      _unknown: unknown,
    })
  }

  pub fn read_txt2(&mut self) -> Result<Txt2> {
    let section = self.read_section()?;
    let string_count = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)? as usize;

    let mut offsets = Vec::with_capacity(string_count);
    let mut raw_strings = Vec::with_capacity(string_count);

    for _ in 0..string_count {
      offsets.push(self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?);
    }

    for i in 0..string_count {
      let next_str_end = if i == string_count - 1 {
        section.size
      } else {
        offsets[i + 1]
      };
      let str_len = next_str_end - offsets[i];
      let mut str_buf = vec![0; str_len as usize];
      self.reader.read_exact(&mut str_buf).map_err(Error::Io)?;
      raw_strings.push(str_buf);
    }

    Ok(Txt2 {
      msbt: NonNull::dangling(),
      section,
      string_count: string_count as u32,
      raw_strings,
    })
  }

  pub fn read_nli1(&mut self) -> Result<Nli1> {
    let section = self.read_section()?;

    let mut map = BTreeMap::default();
    let mut id_count = 0;

    if section.size > 0 {
      id_count = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;

      for _ in 0..id_count {
        let val = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;
        let key = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;
        map.insert(key, val);
      }
    }

    Ok(Nli1 {
      msbt: NonNull::dangling(),
      section,
      id_count,
      global_ids: map,
    })
  }

  pub fn read_group(&mut self) -> Result<Group> {
    let label_count = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;
    let offset = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;

    Ok(Group {
      label_count,
      offset,
    })
  }

  pub fn read_section(&mut self) -> Result<Section> {
    let mut magic = [0; 4];
    let mut padding = [0; 8];

    self.reader.read_exact(&mut magic).map_err(Error::Io)?;
    let size = self.header.endianness.read_u32(&mut self.reader).map_err(Error::Io)?;
    self.reader.read_exact(&mut padding).map_err(Error::Io)?;

    Ok(Section {
      magic,
      size,
      padding,
    })
  }
}

#[derive(Debug)]
pub struct Header {
  pub(crate) magic: [u8; 8],
  pub(crate) endianness: Endianness,
  pub(crate) _unknown_1: u16,
  pub(crate) encoding: Encoding,
  pub(crate) _unknown_2: u8,
  pub(crate) section_count: u16,
  pub(crate) _unknown_3: u16,
  pub(crate) file_size: u32,
  pub(crate) padding: [u8; 10],
}

impl Header {
  pub fn from_reader(mut reader: &mut dyn Read) -> Result<Self> {
    let mut buf = [0u8; 10];
    reader.read_exact(&mut buf[..8]).map_err(Error::Io)?;

    let mut magic = [0u8; 8];
    magic.swap_with_slice(&mut buf[..8]);
    if magic != HEADER_MAGIC {
      return Err(Error::InvalidMagic);
    }

    reader.read_exact(&mut buf[..2]).map_err(Error::Io)?;

    let endianness = if buf[..2] == [0xFE, 0xFF] {
      Endianness::Big
    } else if buf[..2] == [0xFF, 0xFE] {
      Endianness::Little
    } else {
      return Err(Error::InvalidBom);
    };

    let unknown_1 = endianness.read_u16(&mut reader).map_err(Error::Io)?;

    reader.read_exact(&mut buf[..1]).map_err(Error::Io)?;
    let encoding = match buf[0] {
      0x00 => Encoding::Utf8,
      0x01 => Encoding::Utf16,
      x => return Err(Error::InvalidEncoding(x)),
    };

    reader.read_exact(&mut buf[..1]).map_err(Error::Io)?;
    let unknown_2 = buf[0];

    let section_count = endianness.read_u16(&mut reader).map_err(Error::Io)?;

    let unknown_3 = endianness.read_u16(&mut reader).map_err(Error::Io)?;

    let file_size = endianness.read_u32(&mut reader).map_err(Error::Io)?;

    reader.read_exact(&mut buf[..10]).map_err(Error::Io)?;
    let padding = buf;

    Ok(Header {
      magic,
      endianness,
      encoding,
      section_count,
      file_size,
      padding,
      _unknown_1: unknown_1,
      _unknown_2: unknown_2,
      _unknown_3: unknown_3,
    })
  }

  pub fn magic(&self) -> [u8; 8] {
    self.magic
  }

  pub fn endianness(&self) -> Endianness {
    self.endianness
  }

  pub fn unknown_1(&self) -> u16 {
    self._unknown_1
  }

  pub fn encoding(&self) -> Encoding {
    self.encoding
  }

  pub fn unknown_2(&self) -> u8 {
    self._unknown_2
  }

  pub fn section_count(&self) -> u16 {
    self.section_count
  }

  pub fn unknown_3(&self) -> u16 {
    self._unknown_3
  }

  pub fn file_size(&self) -> u32 {
    self.file_size
  }

  pub fn padding(&self) -> [u8; 10] {
    self.padding
  }

  pub(crate) fn calc_file_size(&self) -> usize {
    std::mem::size_of_val(&self.magic)
      + std::mem::size_of::<u16>() // endianness
      + std::mem::size_of_val(&self._unknown_1)
      + std::mem::size_of::<u8>() // encoding
      + std::mem::size_of_val(&self._unknown_2)
      + std::mem::size_of_val(&self.section_count)
      + std::mem::size_of_val(&self._unknown_3)
      + std::mem::size_of_val(&self.file_size)
      + std::mem::size_of_val(&self.padding)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
  Utf8 = 0x00,
  Utf16 = 0x01,
}

