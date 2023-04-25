/* Modules */
pub mod dir;
pub mod file;
pub mod symlink;
/* Built in imports */
use core::result::Result;
use std::{io, path::PathBuf};
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

#[allow(clippy::std_instead_of_core)]
#[derive(thiserror::Error, Debug)]
pub enum FSNodeError {
  #[error("`{0}` - Unknown node type")]
  UnknownNodeType(PathBuf),
  #[error("`{0}` - Not found")]
  NotFound(PathBuf),
  #[error("`{0}` - Not enough permissions")]
  NoPermissions(PathBuf),
  #[error("`{0}` - Not a directory")]
  NotADirectory(PathBuf),
  #[error("`{0}` - Not a symlink")]
  NotASymlink(PathBuf),
  #[error("`{0}` - Modified date not available")]
  ModifiedNotAvailable(PathBuf),
  #[error("`{0}` - Interrupted")]
  Interrupted(PathBuf),
  #[error("`{0}` `{1}` - Unknown error")]
  Unknown(PathBuf, io::ErrorKind),
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
      Err(FSNodeError::UnknownNodeType(path))
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

/* https://doc.rust-lang.org/std/io/enum.ErrorKind.html */
impl FSNodeError {
  pub fn metadata(path: PathBuf, err: &io::Error) -> Self {
    /* https://doc.rust-lang.org/std/fs/fn.metadata.html */
    #[allow(clippy::wildcard_enum_match_arm)]
    match err.kind() {
      /* `path` does not exist */
      io::ErrorKind::NotFound => Self::NotFound(path),
      /* The user lacks permissions to perform `metadata` call on `path` */
      io::ErrorKind::PermissionDenied => Self::NoPermissions(path),
      /* Fallback */
      _ => panic!("{}", Self::Unknown(path, err.kind()).to_string()),
    }
  }

  pub fn read_dir(path: PathBuf, err: &io::Error) -> Self {
    /* https://doc.rust-lang.org/std/fs/fn.read_dir.html */
    #[allow(clippy::wildcard_enum_match_arm)]
    match err.kind() {
      /* The provided `path` doesn’t exist */
      io::ErrorKind::NotFound => Self::NotFound(path),
      /* The process lacks permissions to view the contents */
      io::ErrorKind::PermissionDenied => Self::NoPermissions(path),
      // only available on nightly -- issue #86442
      /* The path points at a non-directory file */
      // io::ErrorKind::NotADirectory => Self::NotADirectory(path),
      /* Fallback */
      _ => panic!("{}", Self::Unknown(path, err.kind()).to_string()),
    }
  }

  pub fn dir_entry(path: PathBuf, err: &io::Error) -> Self {
    /* https://doc.rust-lang.org/std/fs/struct.ReadDir.html */
    #[allow(clippy::wildcard_enum_match_arm)]
    match err.kind() {
      /* This `io::Result` will be an `Err` if there’s some sort of intermittent IO error during iteration. */
      io::ErrorKind::Interrupted => Self::Interrupted(path),
      // TODO: determine if other variants can be returned
      /* Fallback */
      _ => panic!("{}", Self::Unknown(path, err.kind()).to_string()),
    }
  }

  pub fn modified(path: PathBuf, err: &io::Error) -> Self {
    /* https://doc.rust-lang.org/std/fs/struct.Metadata.html#method.modified */
    #[allow(clippy::wildcard_enum_match_arm)]
    match err.kind() {
      /* This field might not be available on all platforms */
      io::ErrorKind::Unsupported => Self::ModifiedNotAvailable(path),
      /* Fallback */
      _ => panic!("{}", Self::Unknown(path, err.kind()).to_string()),
    }
  }

  pub fn read_link(path: PathBuf, err: &io::Error) -> Self {
    /* https://doc.rust-lang.org/std/fs/fn.read_link.html */
    #[allow(clippy::wildcard_enum_match_arm)]
    match err.kind() {
      /* `path` does not exist */
      io::ErrorKind::NotFound => Self::NotFound(path),
      /* `path` is not a symbolic link */
      io::ErrorKind::InvalidInput => Self::NotASymlink(path),
      /* Fallback */
      _ => panic!("{}", Self::Unknown(path, err.kind()).to_string()),
    }
  }
}
