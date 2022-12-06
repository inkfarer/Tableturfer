use std::io::Write;

#[derive(Debug)]
pub struct Counter<W> {
  written: usize,
  writer: W,
}

impl<W: Write> Counter<W> {
  pub fn new(writer: W) -> Self {
    Counter {
      writer,
      written: 0,
    }
  }

  #[allow(unused)]
  pub fn into_inner(self) -> W {
    self.writer
  }

  pub fn written(&self) -> usize {
    self.written
  }
}

impl<W: Write> Write for Counter<W> {
  fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
    let res = self.writer.write(buf)?;
    self.written += res;

    Ok(res)
  }

  fn flush(&mut self) -> std::io::Result<()> {
    self.writer.flush()
  }
}
