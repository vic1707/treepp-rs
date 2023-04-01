use core::result::Result;
use std::{io, path::PathBuf};
use thiserror::Error;

mod dir;
mod file;
mod symlink;

use dir::Dir;
use file::File;
use symlink::SymbolicLink;

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
  pub fn build(path: PathBuf) -> FSNodeRes {
    if path.is_dir() {
      Ok(Self::Directory(Dir::build(path)?))
    } else if path.is_symlink() {
      Ok(Self::SymbolicLink(SymbolicLink::build(path)?))
    } else if path.is_file() {
      Ok(Self::File(File::build(path)?))
    } else {
      Err(FSNodeError::UnknownNodeType(path))
    }
  }
}
