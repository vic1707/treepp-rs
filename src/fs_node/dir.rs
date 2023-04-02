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
    let mut size = 0;

    let entries = fs::read_dir(&path)?
      .map(|entry| {
        let node = FSNode::build(entry?.path());
        if let Ok(ref n) = node {
          size += n.size();
        }
        node
      })
      .collect::<Vec<FSNodeRes>>();

    Ok(Self { path, size, entries })
  }

  pub const fn size(&self) -> i128 {
    self.size
  }

  pub const fn path(&self) -> &PathBuf {
    &self.path
  }

  pub fn entries(&self) -> &[FSNodeRes] {
    &self.entries
  }
}
