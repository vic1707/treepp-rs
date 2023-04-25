/* Built in imports */
use std::{fs, path::PathBuf};
use time::OffsetDateTime;
/* Crate imports */
use super::{FSNode, FSNodeError, FSNodeRes};
use crate::{FilterManager, SorterManager};

pub struct Dir {
  pub path: PathBuf,
  pub size: i128,
  pub entries: Vec<FSNodeRes>,
  pub modified_date: OffsetDateTime,
}

impl Dir {
  pub fn build(
    path: PathBuf,
    filter_manager: &FilterManager,
    sorter_manager: &SorterManager,
  ) -> Result<Self, FSNodeError> {
    let mut size = 0;

    let entries = fs::read_dir(&path)
      .map_err(|ref err| FSNodeError::new(&path, err))?
      .map(|entry| {
        let node = FSNode::build(
          entry
            .map_err(|ref err| FSNodeError::new(&path, err))?
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
      .map_err(|ref err| FSNodeError::new(&path, err))?
      .modified()
      .map_err(|ref err| FSNodeError::new(&path, err))?
      .into();

    Ok(Self {
      path,
      size,
      entries,
      modified_date,
    })
  }
}
