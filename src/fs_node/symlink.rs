/* Built in imports */
use core::result::Result;
use std::{fs, path::PathBuf};
use time::OffsetDateTime;
/* Crate imports */
use super::FSNodeError;

pub struct Symlink {
  pub filename: String,
  pub path: PathBuf,
  pub target_filename: String,
  pub target: PathBuf,
  pub size: i128,
  pub modified_date: OffsetDateTime,
}

impl Symlink {
  pub fn build(path: PathBuf) -> Result<Self, FSNodeError> {
    let metadata = path
      .symlink_metadata()
      .map_err(|ref err| FSNodeError::new(&path, err))?;

    let target =
      fs::read_link(&path).map_err(|ref err| FSNodeError::new(&path, err))?;

    let size = metadata.len().into();
    let modified_date = metadata
      .modified()
      .map_err(|ref err| FSNodeError::new(&path, err))?
      .into();

    let filename = path
      .file_name()
      .ok_or_else(|| FSNodeError::new_no_filename(&path))?
      .to_string_lossy()
      .to_string();

    let target_filename = target
      .file_name()
      .ok_or_else(|| FSNodeError::new_no_filename(&target))?
      .to_string_lossy()
      .to_string();

    Ok(Self {
      filename,
      path,
      target_filename,
      target,
      size,
      modified_date,
    })
  }
}
