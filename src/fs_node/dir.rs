use std::{fs, path::PathBuf};
use time::OffsetDateTime;

use super::{FSNode, FSNodeError, FSNodeRes};
use crate::{FilterManager, SorterManager};

#[derive(Debug)]
pub struct Dir {
  path: PathBuf,
  size: i128,
  entries: Vec<FSNodeRes>,
  modified_date: OffsetDateTime,
}

impl Dir {
  pub fn build(
    path: PathBuf,
    filter_manager: &FilterManager,
    sorter_manager: &SorterManager,
  ) -> Result<Self, FSNodeError> {
    let mut size = 0;

    let entries = fs::read_dir(&path)
      .map_err(|err| FSNodeError::read_dir(path.clone(), &err))?
      .map(|entry| {
        let node = FSNode::build(
          entry
            .map_err(|err| FSNodeError::dir_entry(&path, &err))?
            .path(),
          filter_manager,
          sorter_manager,
        );
        if let Ok(ref n) = node {
          size += n.size();
        }
        node
      })
      .filter(|node| filter_manager.filter(node))
      .collect::<Vec<FSNodeRes>>();

    let modified_date = fs::metadata(&path)
      .map_err(|err| FSNodeError::metadata(path.clone(), &err))?
      .modified()
      .map_err(|err| FSNodeError::modified(path.clone(), &err))?
      .into();

    Ok(Self {
      path,
      size,
      entries,
      modified_date,
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

  pub const fn modified_date(&self) -> &OffsetDateTime {
    &self.modified_date
  }
}
