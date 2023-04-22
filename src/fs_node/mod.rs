use core::result::Result;
use std::{io, path::Path, path::PathBuf};
use thiserror::Error;

use crate::{FilterManager, SorterManager};

mod dir;
mod file;
mod symlink;

pub use dir::Dir;
pub use file::File;
pub use symlink::Symlink;

#[derive(Debug)]
pub enum FSNode {
  File(File),
  Dir(Dir),
  Symlink(Symlink),
}

#[derive(Error, Debug)]
pub enum FSNodeError {
  #[error("`{0}` - Unknown node type")]
  UnknownNodeType(PathBuf),
  #[error("`{0}` - Not found")]
  NotFound(PathBuf),
  #[error("`{0}` - Not enough permissions")]
  NoPermissions(PathBuf),
  #[error("`{0}` - Not a directory")]
  NotADirectory(PathBuf),
  #[error("`{0}` - Modified date not available")]
  ModifiedNotAvailable(PathBuf),
  #[error("`{0}` `{1}` - Unknown error")]
  Unknown(PathBuf, io::ErrorKind),
}

pub type FSNodeRes = Result<FSNode, FSNodeError>;

impl FSNode {
  pub fn build<P: Into<PathBuf>>(
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
      Err(FSNodeError::UnknownNodeType(path))
    }
  }

  pub const fn size(&self) -> &i128 {
    match *self {
      Self::File(ref file) => file.size(),
      Self::Dir(ref dir) => dir.size(),
      Self::Symlink(ref symlink) => symlink.size(),
    }
  }

  pub const fn path(&self) -> &PathBuf {
    match *self {
      Self::File(ref file) => file.path(),
      Self::Dir(ref dir) => dir.path(),
      Self::Symlink(ref symlink) => symlink.path(),
    }
  }

  pub const fn modified_date(&self) -> &time::OffsetDateTime {
    match *self {
      Self::File(ref file) => file.modified_date(),
      Self::Dir(ref dir) => dir.modified_date(),
      Self::Symlink(ref symlink) => symlink.modified_date(),
    }
  }
}

impl FSNodeError {
  pub fn metadata(path: PathBuf, err: &io::Error) -> Self {
    match err.kind() {
      io::ErrorKind::NotFound => Self::NotFound(path),
      io::ErrorKind::PermissionDenied => Self::NoPermissions(path),
      _ => panic!("{}", Self::Unknown(path, err.kind()).to_string())
    }
  }

  pub fn read_dir(path: PathBuf, err: &io::Error) -> Self {
    match err.kind() {
      io::ErrorKind::NotFound => Self::NotFound(path),
      io::ErrorKind::PermissionDenied => Self::NoPermissions(path),
      // only available on nightly -- issue #86442
      // io::ErrorKind::NotADirectory => Self::NotADirectory(path),
      _ => panic!("{}", Self::Unknown(path, err.kind()).to_string())
    }
  }

  pub fn dir_entry(path: PathBuf, err: &io::Error) -> Self {
    panic!("{}", Self::Unknown(path, err.kind()).to_string())
  }

  pub fn modified(path: PathBuf, err: &io::Error) -> Self {
    match err.kind() {
      io::ErrorKind::Unsupported => Self::ModifiedNotAvailable(path),
      _ => panic!("{}", Self::Unknown(path, err.kind()).to_string())
    }
  }

  pub fn read_link(path: PathBuf, err: &io::Error) -> Self {
    match err.kind() {
      io::ErrorKind::NotFound => Self::NotFound(path),
      _ => panic!("{}", Self::Unknown(path, err.kind()).to_string())
    }
  }
}
