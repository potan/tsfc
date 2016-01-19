
use std::fs::File;
use std::fmt::{Debug, Formatter, Error};
use std::io::Result;
pub use std::io::Write;
pub use std::io::Read;
pub use std::path::Path;
pub use std::io::Seek;
pub use std::io::SeekFrom;

pub struct WFile { f: File }
impl WFile {
  pub fn sync_all(&self) -> Result<()> {
   self.f.sync_all()
  }
  pub fn sync_data(&self) -> Result<()> {
   self.f.sync_data()
  }
  pub fn set_len(&self, size: u64) -> Result<()> {
   self.f.set_len(size)
  }
  pub fn metadata(&self) -> Result<std::fs::Metadata> {
   self.f.metadata()
  }
}

pub struct RFile { f: File }
impl RFile {
  pub fn metadata(&self) -> Result<std::fs::Metadata> {
   self.f.metadata()
  }
}


impl Debug for WFile {
  fn fmt(&self, fmtr: &mut Formatter) -> std::result::Result<(), Error> {
   self.f.fmt(fmtr)
  }
}

impl Debug for RFile {
  fn fmt(&self, fmtr: &mut Formatter) -> std::result::Result<(), Error> {
   self.f.fmt(fmtr)
  }
}

impl Seek for RFile {
  fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
   self.f.seek(pos)
  }
}

impl Seek for WFile {
  fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
   self.f.seek(pos)
  }
}

impl Write for WFile {
  fn write(&mut self, buf: &[u8]) -> Result<usize> {
    self.f.write(buf)
  }
  fn flush(&mut self) -> Result<()> {
    self.f.flush()
  }
  fn write_all(&mut self, buf: &[u8]) -> Result<()> {
    self.f.write_all(buf)
  }
  fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<()> {
    self.f.write_fmt(fmt)
  }
  fn by_ref(&mut self) -> &mut Self {
    self
  }
//  fn broadcast<W: Write>(self, other: W) -> std::io::Broadcast<Self, W> {
//    unimplemented!()
//  }
}

impl Read for RFile {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
   self.f.read(buf)
  }
  fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
   self.f.read_to_end(buf)
  }
  fn read_to_string(&mut self, buf: &mut String) -> Result<usize> {
   self.f.read_to_string(buf)
  }
//  fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
//   self.f.read_exact(buf)
//  }
  fn by_ref(&mut self) -> &mut Self {
    self
  }

//  fn bytes(self) -> Bytes<Self> where Self: Sized { ... }
//  fn chars(self) -> Chars<Self> where Self: Sized { ... }
//  fn chain<R: Read>(self, next: R) -> Chain<Self, R> where Self: Sized { ... }
//  fn take(self, limit: u64) -> Take<Self> where Self: Sized { ... }
//  fn tee<W: Write>(self, out: W) -> Tee<Self, W> where Self: Sized { ... }
}

/// Like File::open<P: AsRef<Path>>(path: P) -> Result<File>
/// but don't implement trait std::io:Write
pub fn open<P: AsRef<Path>>(path: P) -> Result<RFile> {
 Ok(RFile {
  f: try!(File::open(path))
 })
}

/// Like File::create<P: AsRef<Path>>(path: P) -> Result<File>
/// but don't implement trait std::io:Read
pub fn create<P: AsRef<Path>>(path: P) -> Result<WFile> {
 Ok(WFile {
  f: try!(File::create(path))
 })
}
