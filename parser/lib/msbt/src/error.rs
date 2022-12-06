use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
  #[error("io error: {0}")]
  Io(std::io::Error),
  #[error("invalid magic bytes")]
  InvalidMagic,
  #[error("invalid BOM")]
  InvalidBom,
  #[error("invalid encoding: {0}")]
  InvalidEncoding(u8),
  #[error("invalid utf-8: {0}")]
  InvalidUtf8(std::string::FromUtf8Error),
  #[error("invalid borrowed utf-8: {0}")]
  InvalidBorrowedUtf8(std::str::Utf8Error),
  #[error("invalid utf-16: {0}")]
  InvalidUtf16(std::string::FromUtf16Error),
  #[error("invalid section header: {0:?}")]
  InvalidSection([u8; 4]),
}
