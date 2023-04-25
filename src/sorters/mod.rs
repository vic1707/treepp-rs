/* Modules */
mod methods;
/* Built in imports */
use core::cmp;
/* Crate imports */
use crate::fs_node::{FSNodeRes, FSNodeError, FSNode};

#[derive(Clone, clap::ValueEnum)]
pub enum Sorter {
  Name,
  Size,
  Modified,
  // FileFolder, // TODO: implement
  // FolderFile, // TODO: implement
  Extension,
}

type SortingMethod = fn(&Result<FSNode, FSNodeError>, &Result<FSNode, FSNodeError>) -> cmp::Ordering;
impl Sorter {
  pub fn get_sorting_method(&self) -> SortingMethod {
    match *self {
      Self::Name => methods::name,
      Self::Size => methods::size,
      Self::Extension => methods::extension,
      Self::Modified => methods::modified_date,
    }
  }
}

pub struct SorterManager {
  sorters: Vec<Sorter>,
}

impl SorterManager {
  pub const fn new(sorters: Vec<Sorter>) -> Self {
    Self { sorters }
  }

  pub fn sort(&self, nodes: &mut [FSNodeRes]) {
    for sorter in &self.sorters {
      let sorting_method = sorter.get_sorting_method();
      nodes.sort_by(sorting_method);
    }
  }
}
