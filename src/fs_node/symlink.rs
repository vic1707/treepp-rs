use core::result::Result;
use std::{fs, path::PathBuf};
use time::OffsetDateTime;

use super::FSNodeError;

#[derive(Debug)]
pub struct SymbolicLink {
  path: PathBuf,
  target: PathBuf,
  size: i128,
  modified_date: OffsetDateTime,
}

impl SymbolicLink {
  pub fn build(path: PathBuf) -> Result<Self, FSNodeError> {
    let metadata = path
      .symlink_metadata()
      .map_err(|err| FSNodeError::metadata(path.clone(), &err))?;

    let target = fs::read_link(&path)
      .map_err(|err| FSNodeError::read_link(path.clone(), &err))?;

    let size = metadata.len().into();
    let modified_date = metadata
      .modified()
      .map_err(|err| FSNodeError::modified(path.clone(), &err))?
      .into();

    Ok(Self {
      path,
      target,
      size,
      modified_date,
    })
  }

  pub const fn size(&self) -> &i128 {
    &self.size
  }

  pub const fn path(&self) -> &PathBuf {
    &self.path
  }

  pub const fn target(&self) -> &PathBuf {
    &self.target
  }

  pub const fn modified_date(&self) -> &OffsetDateTime {
    &self.modified_date
  }
}
