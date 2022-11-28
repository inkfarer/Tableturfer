//! A simple to use library for reading, writing, and converting Nintendo binary YAML (BYML) files in
//! Rust. Supports BYML versions 2-4, (v2 used in *The Legend of Zelda: Breath of the Wild*). Can
//! convert from BYML to readable, editable YAML and back.
//!
//! Sample usage:
//!
//! ```
//! use byml::Byml;
//! // First grab the file bytes. Yaz0 compressed files are automatically decompressed.
//! let bytes: Vec<u8> = std::fs::read("test/ActorInfo.product.byml").unwrap();
//! // Parse the data as a Byml document
//! let actor_info: Byml = Byml::from_binary(&bytes).unwrap();
//! // Index BYML hashes and arrays naturally
//! let actor_list: &Vec<Byml> = actor_info["Actors"].as_array().unwrap();
//! // 7934 actors, egads!
//! assert_eq!(actor_list.len(), 7934);
//! // Hmm, we'll iterate the actors listed in this file:
//! for actor in actor_list.iter() {
//!     // Print each actor's name
//!     println!("{}", actor["name"].as_string().unwrap());
//! }
//! // Dump to YAML
//! std::fs::write("test/ActorInfo.product.yml", actor_info.to_text().unwrap()).unwrap();
//! ```
use binread::BinRead;
use std::collections::BTreeMap;
use std::error::Error;

mod parse;

type AnyError = Box<dyn Error>;

/// Specifies endianness for binary BYML operations
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Endian {
    Big,
    Little,
}

impl From<binread::Endian> for Endian {
    fn from(endian: binread::Endian) -> Endian {
        match endian {
            binread::Endian::Big => Endian::Big,
            binread::Endian::Little => Endian::Little,
            _ => unimplemented!(),
        }
    }
}

/// Error thrown when trying to get BYML as incorrect variant
#[derive(Debug)]
pub struct TypeError;

impl Error for TypeError {}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Incorrect type ")
    }
}

/// An enumeration of valid BYML node types
#[repr(u8)]
#[derive(Debug, BinRead, PartialEq)]
pub enum NodeType {
    String = 0xA0,
    Binary = 0xA1,
    Array = 0xC0,
    Hash = 0xC1,
    /// Note: the main `Byml` enum does not include a corresponding representation for
    /// `NodeType::StringTable`, as string table nodes are only used internally for binary
    /// (de)serialization.
    StringTable = 0xC2,
    Bool = 0xD0,
    Int = 0xD1,
    Float = 0xD2,
    UInt = 0xD3,
    Int64 = 0xD4,
    UInt64 = 0xD5,
    Double = 0xD6,
    Null = 0xFF,
}

#[derive(Debug, PartialEq)]
struct U24(u64);
/// Wrapper type to preserve f32 values with `Eq` and related traits. Implements `From<f32>` and
/// `Into<f32>`.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub struct Float(u32, Endian);
/// Wrapper type to preserve f64 values with `Eq` and related traits. Implements `From<f64>` and
/// `Into<f64>`.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub struct Double(u64, Endian);

impl From<f32> for Float {
    fn from(float: f32) -> Self {
        Self(u32::from_be_bytes(float.to_be_bytes()), Endian::Big)
    }
}

impl Into<f32> for &Float {
    fn into(self) -> f32 {
        match self.1 {
            Endian::Big => f32::from_be_bytes(self.0.to_be_bytes()),
            Endian::Little => f32::from_le_bytes(self.0.to_le_bytes()),
        }
    }
}

impl From<f64> for Double {
    fn from(dbl: f64) -> Self {
        Self(u64::from_be_bytes(dbl.to_be_bytes()), Endian::Big)
    }
}

impl Into<f64> for &Double {
    fn into(self) -> f64 {
        match self.1 {
            Endian::Big => f64::from_be_bytes(self.0.to_be_bytes()),
            Endian::Little => f64::from_le_bytes(self.0.to_le_bytes()),
        }
    }
}

/// Represents a Nintendo binary YAML (BYML) document or node. A `Byml` will usually be constructed
/// from binary data or a YAML string, e.g.
/// ```
/// # use byml::Byml;
/// # use std::{fs::read, error::Error};
/// # fn docttest() -> Result<(), Box<dyn Error>> {
/// let buf: Vec<u8> = std::fs::read("A-1_Static.smubin")?;
/// let map_unit = Byml::from_binary(&buf)?;
/// let text: String = std::fs::read_to_string("A-1_Static.yml")?;
/// let map_unit2 = Byml::from_text(&text)?;
/// assert_eq!(map_unit, map_unit2);
/// # Ok(())
/// # }
/// ```
/// You can also easily serialize to binary or a YAML string.
/// ```
/// # use byml::{Byml, Endian};
/// # fn docttest() -> Result<(), Box<dyn std::error::Error>> {
/// let buf: Vec<u8> = std::fs::read("A-1_Static.smubin")?;
/// let map_unit = Byml::from_binary(&buf)?;
/// std::fs::write("A-1_Static.yml", &map_unit.to_text()?)?;
/// std::fs::write("A-1_Static.copy.mubin", &map_unit.to_binary(Endian::Big, 2)?)?;
/// # Ok(())
/// # }
/// ```
///
/// A number of convenience getters are available which return a result for a variant value:
/// ```
/// # use byml::Byml;
/// # use std::collections::BTreeMap;
/// # fn docttest() -> Result<(), Box<dyn std::error::Error>> {
/// # let some_data = b"";
/// let doc = Byml::from_binary(&some_data)?;
/// let hash: &BTreeMap<String, Byml> = doc.as_hash()?;
/// # Ok(())
/// # }
/// ```
///
/// Most of the node types are fairly self-explanatory. Arrays are implemented as `Vec<Byml>`, and
/// hash nodes as `BTreeMap<String, Byml>`. Floats (f32) and doubles (f64) use wrapper types that
/// support `Eq`. These can be converted with `into()`. You can also query the node type with
/// `get_type()`.
///
/// For convenience, a `Byml` *known* to be an array or hash node can be indexed. **Panics if the
/// node has the wrong type or if the index is not found**.
/// ```
/// # use byml::Byml;
/// # fn docttest() -> Result<(), Box<dyn std::error::Error>> {
/// let buf: Vec<u8> = std::fs::read("ActorInfo.product.sbyml")?;
/// let actor_info = Byml::from_binary(&buf)?;
/// assert_eq!(actor_info["Actors"].as_array()?.len(), 7934);
/// assert_eq!(actor_info["Hashes"][0].as_int()?, 31119);
/// # Ok(())
/// # }
/// ```
#[allow(clippy::clippy::clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, Eq, Hash)]
pub enum Byml {
    Null,
    String(String),
    Binary(Vec<u8>),
    Array(Vec<Byml>),
    Hash(BTreeMap<String, Byml>),
    Bool(bool),
    Int(i32),
    Float(Float),
    UInt(u32),
    Int64(i64),
    UInt64(u64),
    Double(Double),
}

impl Default for Byml {
    fn default() -> Self {
        Self::Null
    }
}

impl PartialEq for Byml {
    fn eq(&self, other: &Byml) -> bool {
        match self {
            Byml::Array(a) => match other.as_array() {
                Ok(a2) => a == a2,
                Err(_) => false,
            },
            Byml::Hash(h) => match other.as_hash() {
                Ok(h2) => h == h2,
                Err(_) => false,
            },
            Byml::Binary(v) => match other.as_binary() {
                Ok(v2) => v == v2,
                Err(_) => false,
            },
            Byml::Bool(v) => match other.as_bool() {
                Ok(v2) => *v == v2,
                Err(_) => false,
            },
            Byml::Double(v) => match other.as_double() {
                Ok(v2) => {
                    let v1: f64 = v.into();
                    v1 == v2
                }
                Err(_) => false,
            },
            Byml::Float(v) => match other.as_float() {
                Ok(v2) => {
                    let v1: f32 = v.into();
                    v1 == v2
                }
                Err(_) => false,
            },
            Byml::Int(v) => match other.as_int() {
                Ok(v2) => v == &v2,
                Err(_) => false,
            },
            Byml::Int64(v) => match other.as_int64() {
                Ok(v2) => v == &v2,
                Err(_) => false,
            },
            Byml::UInt(v) => match other.as_uint() {
                Ok(v2) => v == &v2,
                Err(_) => false,
            },
            Byml::UInt64(v) => match other.as_uint64() {
                Ok(v2) => v == &v2,
                Err(_) => false,
            },
            Byml::String(v) => match other.as_string() {
                Ok(v2) => v == v2,
                Err(_) => false,
            },
            Byml::Null => other.is_null(),
        }
    }
}

/// Convenience type for indexing a hash or array BYML node
pub enum BymlIndex<'a> {
    Key(&'a str),
    Index(usize),
}

impl From<&'static str> for BymlIndex<'_> {
    fn from(key: &'static str) -> BymlIndex<'_> {
        BymlIndex::Key(key)
    }
}

impl From<usize> for BymlIndex<'_> {
    fn from(idx: usize) -> BymlIndex<'static> {
        BymlIndex::Index(idx)
    }
}

impl<'a, I> std::ops::Index<I> for Byml
where
    I: Into<BymlIndex<'a>>,
{
    type Output = Byml;
    fn index(&self, index: I) -> &Self::Output {
        let idx = index.into();
        match idx {
            BymlIndex::Key(k) => &self.as_hash().unwrap()[k],
            BymlIndex::Index(i) => &self.as_array().unwrap()[i],
        }
    }
}

impl Byml {
    /// Returns whether the node is an array or hash
    pub fn is_container(&self) -> bool {
        matches! (self, Byml::Hash(_) | Byml::Array(_))
    }

    /// Returns whether the node is an inline value (`Int`, `UInt`, `Float`, or `Bool`)
    pub fn is_value(&self) -> bool {
        matches! (self, Byml::Int(_) | Byml::UInt(_) | Byml::Float(_) | Byml::Bool(_))
    }

    /// Do I even need to document this one?
    pub fn is_string(&self) -> bool {
        matches! (self, Byml::String(_))
    }

    /// Gets the node type
    pub fn get_type(&self) -> NodeType {
        match self {
            Byml::Array(_) => NodeType::Array,
            Byml::Hash(_) => NodeType::Hash,
            Byml::Binary(_) => NodeType::Binary,
            Byml::Bool(_) => NodeType::Bool,
            Byml::Double(_) => NodeType::Double,
            Byml::Float(_) => NodeType::Float,
            Byml::Int(_) => NodeType::Int,
            Byml::Int64(_) => NodeType::Int64,
            Byml::Null => NodeType::Null,
            Byml::String(_) => NodeType::String,
            Byml::UInt(_) => NodeType::UInt,
            Byml::UInt64(_) => NodeType::UInt64,
        }
    }

    /// Returns a result with a reference to the inner BYML hash or a type error
    pub fn as_hash(&self) -> Result<&BTreeMap<String, Byml>, TypeError> {
        match self {
            Byml::Hash(v) => Ok(&v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a reference to the inner BYML array or a type error
    pub fn as_array(&self) -> Result<&Vec<Byml>, TypeError> {
        match self {
            Byml::Array(v) => Ok(&v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a reference to the inner BYML binary data or a type error
    pub fn as_binary(&self) -> Result<&Vec<u8>, TypeError> {
        match self {
            Byml::Binary(v) => Ok(&v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with the inner boolean value or a type error
    pub fn as_bool(&self) -> Result<bool, TypeError> {
        match self {
            Byml::Bool(v) => Ok(*v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a reference to the inner string or a type error
    pub fn as_string(&self) -> Result<&String, TypeError> {
        match self {
            Byml::String(v) => Ok(&v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with the inner i32 value or a type error
    pub fn as_int(&self) -> Result<i32, TypeError> {
        match self {
            Byml::Int(v) => Ok(*v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with the inner i64 value or a type error
    pub fn as_int64(&self) -> Result<i64, TypeError> {
        match self {
            Byml::Int64(v) => Ok(*v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with the inner u32 value or a type error
    pub fn as_uint(&self) -> Result<u32, TypeError> {
        match self {
            Byml::UInt(v) => Ok(*v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with the inner u64 value or a type error
    pub fn as_uint64(&self) -> Result<u64, TypeError> {
        match self {
            Byml::UInt64(v) => Ok(*v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with the inner f32 value or a type error
    pub fn as_float(&self) -> Result<f32, TypeError> {
        match self {
            Byml::Float(v) => Ok(v.into()),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with the inner f64 value or a type error
    pub fn as_double(&self) -> Result<f64, TypeError> {
        match self {
            Byml::Double(v) => Ok(v.into()),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a mutable reference to the inner BYML hash or a type error
    pub fn as_mut_hash(&mut self) -> Result<&mut BTreeMap<String, Byml>, TypeError> {
        match self {
            Byml::Hash(v) => Ok(v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a mutable reference to the inner BYML array or a type error
    pub fn as_mut_array(&mut self) -> Result<&mut Vec<Byml>, TypeError> {
        match self {
            Byml::Array(v) => Ok(v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a mutable reference to the inner binary data or a type error
    pub fn as_mut_binary(&mut self) -> Result<&mut Vec<u8>, TypeError> {
        match self {
            Byml::Binary(v) => Ok(v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a mutable reference to the inner string or a type error
    pub fn as_mut_string(&mut self) -> Result<&mut String, TypeError> {
        match self {
            Byml::String(v) => Ok(v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a mutable reference to the inner i32 or a type error
    pub fn as_mut_int(&mut self) -> Result<&mut i32, TypeError> {
        match self {
            Byml::Int(v) => Ok(v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a mutable reference to the inner i64 or a type error
    pub fn as_mut_int64(&mut self) -> Result<&mut i64, TypeError> {
        match self {
            Byml::Int64(v) => Ok(v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a mutable reference to the inner u32 or a type error
    pub fn as_mut_uint(&mut self) -> Result<&mut u32, TypeError> {
        match self {
            Byml::UInt(v) => Ok(v),
            _ => Err(TypeError),
        }
    }

    /// Returns a result with a mutable reference to the inner u64 or a type error
    pub fn as_mut_uint64(&mut self) -> Result<&mut u64, TypeError> {
        match self {
            Byml::UInt64(v) => Ok(v),
            _ => Err(TypeError),
        }
    }

    /// Checks if the node is a null value
    pub fn is_null(&self) -> bool {
        matches! (self, Byml::Null)
    }
}

#[cfg(test)]
mod tests {
    use crate::Byml;
    use glob::glob;
    use std::fs::{read, read_to_string};
    use std::path::PathBuf;

    #[test]
    fn parse_byml() {
        let data = read("test/ActorInfo.product.byml").unwrap();
        let actorinfo = Byml::from_binary(&data).unwrap();
        println!("{:?}", actorinfo["Actors"][1]);
        assert_eq!(actorinfo["Actors"].as_array().unwrap().len(), 7934);
        let data = read("test/A-1_Static.mubin.byml").unwrap();
        Byml::from_binary(&data).unwrap();
    }
}
