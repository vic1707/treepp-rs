use core::result::Result;
use std::{path::PathBuf, time::SystemTime};

use super::FSNodeError;

#[derive(Debug)]
pub struct File {
  path: PathBuf,
  size: i128,
  // TODO: better date handling
  modified_date: SystemTime,
}

impl File {
  pub fn build(path: PathBuf) -> Result<Self, FSNodeError> {
    let metadata = path.metadata()?;
    let modified_date = metadata.modified()?;
    Ok(Self {
      path,
      size: metadata.len().into(),
      modified_date,
    })
  }

  pub const fn size(&self) -> i128 {
    self.size
  }
}
