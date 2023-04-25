/* Built in imports */
use core::result::Result;
use std::path::PathBuf;
use time::OffsetDateTime;
/* Crate imports */
use super::FSNodeError;

pub struct File {
  pub path: PathBuf,
  pub size: i128,
  pub modified_date: OffsetDateTime,
}

impl File {
  pub fn build(path: PathBuf) -> Result<Self, FSNodeError> {
    let metadata = path
      .metadata()
      .map_err(|ref err| FSNodeError::new(&path, err))?;
    let modified_date = metadata
      .modified()
      .map_err(|ref err| FSNodeError::new(&path, err))?
      .into();
    Ok(Self {
      path,
      size: metadata.len().into(),
      modified_date,
    })
  }
}
