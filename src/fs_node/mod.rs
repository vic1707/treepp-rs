use core::result::Result;
use std::{io, path::PathBuf};
use thiserror::Error;

mod dir;
mod file;
mod symlink;

pub use dir::Dir;
pub use file::File;
pub use symlink::SymbolicLink;

#[derive(Debug)]
pub enum FSNode {
  File(File),
  Directory(Dir),
  SymbolicLink(SymbolicLink),
}

#[derive(Error, Debug)]
pub enum FSNodeError {
  #[error("`{0}` - Unknown node type")]
  UnknownNodeType(PathBuf),
  #[error("`{0}`")]
  IOError(#[from] io::Error),
}

pub type FSNodeRes = Result<FSNode, FSNodeError>;

impl FSNode {
  pub fn build<P: Into<PathBuf>>(path_: P) -> FSNodeRes {
    let path: PathBuf = path_.into();
    if path.is_symlink() {
      Ok(Self::SymbolicLink(SymbolicLink::build(path)?))
    } else if path.is_dir() {
      Ok(Self::Directory(Dir::build(path)?))
    } else if path.is_file() {
      Ok(Self::File(File::build(path)?))
    } else {
      Err(FSNodeError::UnknownNodeType(path))
    }
  }

  pub const fn size(&self) -> i128 {
    match *self {
      Self::File(ref file) => file.size(),
      Self::Directory(ref dir) => dir.size(),
      Self::SymbolicLink(ref symlink) => symlink.size(),
    }
  }

  pub const fn path(&self) -> &PathBuf {
    match *self {
      Self::File(ref file) => file.path(),
      Self::Directory(ref dir) => dir.path(),
      Self::SymbolicLink(ref symlink) => symlink.path(),
    }
  }
}
