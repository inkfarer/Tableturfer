use crate::{AnyError, Byml, NodeType, U24};
use binread::{BinRead, BinReaderExt, Endian, NullString};
use byteorder::ByteOrder;
use std::io::{Cursor, Read, Seek, SeekFrom};

type BymlResult = Result<Byml, AnyError>;

impl From<u8> for NodeType {
    fn from(val: u8) -> NodeType {
        match val {
            0xA0 => NodeType::String,
            0xA1 => NodeType::Binary,
            0xC0 => NodeType::Array,
            0xC1 => NodeType::Hash,
            0xD0 => NodeType::Bool,
            0xD1 => NodeType::Int,
            0xD2 => NodeType::Float,
            0xD3 => NodeType::UInt,
            0xD4 => NodeType::Int64,
            0xD5 => NodeType::UInt64,
            0xD6 => NodeType::Double,
            0xFF => NodeType::Null,
            _ => panic!("Invalid node type"),
        }
    }
}

#[derive(Debug, BinRead)]
#[br(assert([b"BY", b"YB"].contains(&&magic)))]
struct BymlDoc {
    magic: [u8; 2],
    #[br(is_big = (&magic == b"BY"), is_little = (&magic == b"YB"))]
    header: Header,
}

#[derive(Debug, BinRead)]
#[br(assert(version >= 2 && version < 8))]
struct Header {
    #[allow(dead_code)]
    version: u16,
    hash_table_offset: u32,
    string_table_offset: u32,
    root_node_offset: u32,
}

#[derive(Debug, BinRead)]
#[br(magic = 0xC2u8, assert(strings.len() as u64 == entries.0))]
struct StringTable {
    #[allow(dead_code)]
    entries: U24,
    #[allow(dead_code)]
    #[br(count = entries.0)]
    offsets: Vec<u32>,
    #[br(parse_with = parse_string_table, args(offsets.clone()))]
    strings: Vec<String>,
}

fn parse_string_table<R: binread::io::Read + binread::io::Seek>(
    reader: &mut R,
    _: &binread::ReadOptions,
    args: (Vec<u32>,),
) -> binread::BinResult<Vec<String>> {
    let mut strings: Vec<String> = vec![];
    let base_offset: u64 = reader.seek(SeekFrom::Current(0))? - 4 - (4 * args.0.len() as u64);
    for offset in args.0 {
        let abs: u64 = base_offset + (offset as u64);
        reader.seek(SeekFrom::Start(abs))?;
        strings.push(NullString::read(reader)?.to_string());
    }
    Ok(strings)
}

impl Byml {
    pub fn from_binary<B: AsRef<[u8]>>(data: &B) -> BymlResult {
        let data = data.as_ref();
        if &data[0..4] == b"Yaz0" {
            let mut yaz = yaz0::Yaz0Archive::new(Cursor::new(data))?;
            Byml::read_binary(&mut Cursor::new(yaz.decompress()?))
        } else {
            Byml::read_binary(&mut Cursor::new(data))
        }
    }

    pub fn read_binary<R: Read + Seek>(reader: &mut R) -> BymlResult {
        let mut parser = BymlParser::new(reader)?;
        parser.parse()
    }
}

struct BymlParser<'a, R: Read + Seek> {
    endian: Endian,
    hash_strings: Vec<String>,
    value_strings: Vec<String>,
    root_node_offset: u32,
    reader: &'a mut R,
}

impl<R: Read + Seek> BymlParser<'_, R> {
    fn new(reader: &mut R) -> Result<BymlParser<R>, AnyError> {
        let doc: BymlDoc = BymlDoc::read(reader)?;
        let endian = match &doc.magic {
            b"BY" => Endian::Big,
            b"YB" => Endian::Little,
            _ => unreachable!(),
        };
        let mut opts = binread::ReadOptions::default();
        opts.endian = endian;
        reader.seek(SeekFrom::Start(doc.header.hash_table_offset.into()))?;
        let hash_strings: Vec<String> = match StringTable::read_options(reader, &opts, ()) {
            Ok(s) => s.strings,
            Err(_) => vec![],
        };
        reader.seek(SeekFrom::Start(doc.header.string_table_offset.into()))?;
        let value_strings: Vec<String> = match StringTable::read_options(reader, &opts, ()) {
            Ok(s) => s.strings,
            Err(_) => vec![],
        };
        Ok(BymlParser {
            endian,
            hash_strings,
            value_strings,
            root_node_offset: doc.header.root_node_offset,
            reader,
        })
    }

    fn read<B: BinRead>(&mut self) -> Result<B, binread::Error> {
        match self.endian {
            Endian::Big => self.reader.read_be(),
            Endian::Little => self.reader.read_le(),
            _ => unreachable!(),
        }
    }

    fn align(&mut self) -> Result<(), AnyError> {
        let pos = self.reader.stream_position()?;
        self.reader.seek(SeekFrom::Start(((pos + 4 - 1) / 4) * 4))?;
        Ok(())
    }

    fn parse(&mut self) -> BymlResult {
        self.reader
            .seek(SeekFrom::Start(self.root_node_offset as u64))?;
        let node_type: NodeType = self.read::<u8>()?.into();
        self.parse_node_with_type(&node_type, 12)
    }

    fn parse_node(&mut self, offset: u32) -> BymlResult {
        self.reader.seek(SeekFrom::Start(offset.into()))?;
        let node_type: NodeType = self.read::<u8>()?.into();
        self.parse_node_with_type(&node_type, offset + 1)
    }

    fn parse_node_with_type(&mut self, node_type: &NodeType, offset: u32) -> BymlResult {
        self.reader.seek(SeekFrom::Start(offset.into()))?;
        Ok(match node_type {
            NodeType::String => Byml::String({
                let idx = self.read::<u32>()?;
                self.value_strings[idx as usize].to_owned()
            }),
            NodeType::Int => Byml::Int(self.read::<i32>()?),
            NodeType::UInt => Byml::UInt(self.read::<u32>()?),
            NodeType::Float => Byml::Float(crate::Float(self.read::<u32>()?, self.endian.into())),
            NodeType::Bool => Byml::Bool(self.read::<u32>()? != 0),
            NodeType::Array => {
                let offset = self.read::<u32>()?;
                self.parse_array(offset)?
            }
            NodeType::Hash => {
                let offset = self.read::<u32>()?;
                self.parse_hash(offset)?
            }
            NodeType::Int64 => {
                let offset = self.read::<u32>()?;
                Byml::Int64(self.read_long(offset)? as i64)
            }
            NodeType::UInt64 => {
                let offset = self.read::<u32>()?;
                Byml::UInt64(self.read_long(offset)?)
            }
            NodeType::Double => {
                let offset = self.read::<u32>()?;
                Byml::Double(crate::Double(self.read_long(offset)?, self.endian.into()))
            }
            NodeType::Binary => {
                let offset = self.read::<u32>()?;
                self.parse_binary(offset)?
            }
            NodeType::StringTable => unreachable!(),
            NodeType::Null => Byml::Null,
        })
    }

    fn parse_binary(&mut self, offset: u32) -> BymlResult {
        self.reader.seek(SeekFrom::Start(offset.into()))?;
        let size = self.read::<u32>()?;
        let mut opts = binread::ReadOptions::default();
        opts.endian = self.endian;
        opts.count = Some(size as usize);
        Ok(Byml::Binary(Vec::<u8>::read_options(
            self.reader,
            &opts,
            (),
        )?))
    }

    fn read_long(&mut self, offset: u32) -> Result<u64, binread::Error> {
        self.reader.seek(SeekFrom::Start(offset.into()))?;
        self.read::<u64>()
    }

    fn parse_hash(&mut self, offset: u32) -> BymlResult {
        self.reader.seek(SeekFrom::Start(offset.into()))?;
        let header: HashHeader = self.read()?;
        let pos = self.reader.stream_position()?;
        let hash: std::collections::BTreeMap<String, Byml> = (0..header.entries)
            .map(|i| {
                self.reader.seek(SeekFrom::Start(pos + i as u64 * 8))?;
                let idx: u32 = self.read::<U24>()?.0 as u32;
                Ok((
                    self.hash_strings[idx as usize].to_owned(),
                    self.parse_node(pos as u32 + i * 8 + 3)?,
                ))
            })
            .collect::<Result<std::collections::BTreeMap<String, Byml>, AnyError>>()?;
        Ok(Byml::Hash(hash))
    }

    fn parse_array(&mut self, offset: u32) -> BymlResult {
        self.reader.seek(SeekFrom::Start(offset.into()))?;
        let header: ArrayHeader = self.read()?;
        self.align()?;
        let val_start = self.reader.stream_position()?;
        let array: Vec<Byml> = header
            .node_types
            .iter()
            .enumerate()
            .map(|(i, t)| self.parse_node_with_type(t, val_start as u32 + (i as u32 * 4)))
            .collect::<Result<Vec<Byml>, AnyError>>()?;
        Ok(Byml::Array(array))
    }
}

#[derive(Debug, BinRead)]
#[br(assert(magic == 0xC0u8))]
struct ArrayHeader {
    #[allow(dead_code)]
    magic: u8,
    #[allow(dead_code)]
    #[br(map = |x: U24| x.0 as u32)]
    entries: u32,
    #[br(
        count = entries,
        map = |x: Vec<u8>| x.into_iter().map(|t: u8| NodeType::from(t)).collect()
    )]
    node_types: Vec<NodeType>,
}

#[derive(Debug, BinRead)]
#[br(magic = 0xC1u8)]
struct HashHeader {
    #[br(map = |x: U24| x.0 as u32)]
    entries: u32,
}

impl BinRead for U24 {
    type Args = ();
    fn read_options<R: binread::io::Seek + binread::io::Read>(
        reader: &mut R,
        options: &binread::ReadOptions,
        _: (),
    ) -> binread::BinResult<U24> {
        let buf: [u8; 3] = <[u8; 3]>::read(reader)?;
        match options.endian {
            binread::Endian::Big => Ok(U24(byteorder::BigEndian::read_uint(&buf, 3))),
            binread::Endian::Little => Ok(U24(byteorder::LittleEndian::read_uint(&buf, 3))),
            _ => unreachable!(),
        }
    }
}
