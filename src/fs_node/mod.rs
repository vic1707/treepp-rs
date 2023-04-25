/* Modules */
pub mod dir;
pub mod file;
pub mod symlink;
/* Built in imports */
use core::result::Result;
use std::{
  io,
  path::{Path, PathBuf},
};
/* Crate imports */
use crate::{FilterManager, SorterManager};
use dir::Dir;
use file::File;
use symlink::Symlink;

pub enum FSNode {
  File(File),
  Dir(Dir),
  Symlink(Symlink),
}

#[derive(thiserror::Error, Debug)]
#[error("`{filename}` - {source}")]
pub struct FSNodeError {
  filename: String,
  source: FSNodeErrorKind,
}

#[derive(thiserror::Error, Debug)]
pub enum FSNodeErrorKind {
  #[error("Unknown node type")]
  UnknownNodeType,
  #[error("{0}")]
  IoError(io::ErrorKind),
}

pub type FSNodeRes = Result<FSNode, FSNodeError>;

impl FSNode {
  pub fn build<P: Into<PathBuf>>(
    // TODO: can we remove the `Into` trait bound?
    path_: P,
    filter_manager: &FilterManager,
    sorter_manager: &SorterManager,
  ) -> FSNodeRes {
    let path: PathBuf = path_.into();
    if path.is_symlink() {
      Ok(Self::Symlink(Symlink::build(path)?))
    } else if path.is_dir() {
      Ok(Self::Dir(Dir::build(path, filter_manager, sorter_manager)?))
    } else if path.is_file() {
      Ok(Self::File(File::build(path)?))
    } else {
      Err(FSNodeError {
        filename: path.to_string_lossy().to_string(),
        source: FSNodeErrorKind::UnknownNodeType,
      })
    }
  }

  pub const fn size(&self) -> &i128 {
    match *self {
      Self::File(ref file) => &file.size,
      Self::Dir(ref dir) => &dir.size,
      Self::Symlink(ref symlink) => &symlink.size,
    }
  }

  pub const fn path(&self) -> &PathBuf {
    match *self {
      Self::File(ref file) => &file.path,
      Self::Dir(ref dir) => &dir.path,
      Self::Symlink(ref symlink) => &symlink.path,
    }
  }

  pub const fn modified_date(&self) -> &time::OffsetDateTime {
    match *self {
      Self::File(ref file) => &file.modified_date,
      Self::Dir(ref dir) => &dir.modified_date,
      Self::Symlink(ref symlink) => &symlink.modified_date,
    }
  }
}

impl FSNodeError {
  pub fn new(path: &Path, err: &io::Error) -> Self {
    Self {
      filename: path.to_string_lossy().to_string(),
      source: FSNodeErrorKind::IoError(err.kind()),
    }
  }
}
