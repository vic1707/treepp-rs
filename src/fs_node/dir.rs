use std::{fs, path::PathBuf};

use super::{FSNode, FSNodeError, FSNodeRes};

#[derive(Debug)]
pub struct Dir {
  path: PathBuf,
  size: i128,
  entries: Vec<FSNodeRes>,
}

impl Dir {
  pub fn build(path: PathBuf) -> Result<Self, FSNodeError> {
    let entries = fs::read_dir(&path)?
      .map(|entry| FSNode::build(entry?.path()))
      .collect::<Vec<FSNodeRes>>();

    // TODO: calculate size of directory
    let size = 0;

    Ok(Self { path, size, entries })
  }
}
