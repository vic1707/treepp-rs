use core::result::Result;
use std::{fs, path::PathBuf};

use super::FSNodeError;

#[derive(Debug)]
pub struct SymbolicLink {
  path: PathBuf,
  target: PathBuf,
  size: i128,
}

impl SymbolicLink {
  pub fn build(path: PathBuf) -> Result<Self, FSNodeError> {
    let target = fs::read_link(&path)?;
    let size = path.symlink_metadata()?.len().into();
    Ok(Self { path, target, size })
  }

  pub const fn size(&self) -> i128 {
    self.size
  }

  pub const fn path(&self) -> &PathBuf {
    &self.path
  }

  pub const fn target(&self) -> &PathBuf {
    &self.target
  }
}
