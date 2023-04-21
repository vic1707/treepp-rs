use core::result::Result;
use std::path::PathBuf;
use time::OffsetDateTime;

use super::FSNodeError;

#[derive(Debug)]
pub struct File {
  path: PathBuf,
  size: i128,
  modified_date: OffsetDateTime,
}

impl File {
  pub fn build(path: PathBuf) -> Result<Self, FSNodeError> {
    let metadata = path
      .metadata()
      .map_err(|ref err| FSNodeError::metadata(path.clone(), err))?;
    let modified_date = metadata
      .modified()
      .map_err(|ref err| FSNodeError::modified(path.clone(), err))?
      .into();
    Ok(Self {
      path,
      size: metadata.len().into(),
      modified_date,
    })
  }

  pub const fn size(&self) -> &i128 {
    &self.size
  }

  pub const fn path(&self) -> &PathBuf {
    &self.path
  }

  pub const fn modified_date(&self) -> &OffsetDateTime {
    &self.modified_date
  }
}
