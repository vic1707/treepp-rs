use std::{fs, path::PathBuf};

use super::{FSNode, FSNodeError, FSNodeRes};
use crate::{FilterManager, SorterManager};

#[derive(Debug)]
pub struct Dir {
  path: PathBuf,
  size: i128,
  entries: Vec<FSNodeRes>,
}

impl Dir {
  pub fn build(
    path: PathBuf,
    filter_manager: &FilterManager,
    sorter_manager: &SorterManager,
  ) -> Result<Self, FSNodeError> {
    let mut size = 0;

    let entries = fs::read_dir(&path)?
      .map(|entry| {
        let node = FSNode::build(entry?.path(), filter_manager, sorter_manager);
        if let Ok(ref n) = node {
          size += n.size();
        }
        node
      })
      .filter(|node| filter_manager.filter(node))
      .collect::<Vec<FSNodeRes>>();

    Ok(Self {
      path,
      size,
      entries,
    })
  }

  pub const fn size(&self) -> &i128 {
    &self.size
  }

  pub const fn path(&self) -> &PathBuf {
    &self.path
  }

  pub fn entries(&self) -> &[FSNodeRes] {
    &self.entries
  }

  pub fn entries_mut(&mut self) -> &mut Vec<FSNodeRes> {
    &mut self.entries
  }
}
