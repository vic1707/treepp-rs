use core::result::Result;
use std::{fs, path::PathBuf};

use super::FSNodeError;

#[derive(Debug)]
pub struct SymbolicLink {
  path: PathBuf,
  target: PathBuf,
}

impl SymbolicLink {
  pub fn build(path: PathBuf) -> Result<Self, FSNodeError> {
    let target = fs::read_link(&path)?;
    Ok(Self { path, target })
  }
}
