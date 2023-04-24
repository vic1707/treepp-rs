use core::cmp;

use crate::fs_node::FSNodeRes;

mod methods;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Sorter {
  Name,
  Size,
  Modified,
  FileFolder,
  FolderFile,
  Extension,
}

impl Sorter {
  pub fn sort(&self, n1: &FSNodeRes, n2: &FSNodeRes) -> cmp::Ordering {
    match *self {
      Self::Name => methods::name(n1, n2),
      Self::Size => methods::size(n1, n2),
      Self::Extension => methods::extension(n1, n2),
      Self::Modified => methods::modified_date(n1, n2),
      Self::FileFolder | Self::FolderFile => todo!(),
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
      nodes.sort_by(|n1, n2| sorter.sort(n1, n2));
    }
  }
}
