use core::cmp;

use crate::fs_node::FSNodeRes;

mod methods;

// TODO: not a big fan of Clone if I can avoid it
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
      Self::Modified | Self::FileFolder | Self::FolderFile => todo!(),
    }
  }
}

pub struct SorterManager<'sorters> {
  sorters: &'sorters [Sorter],
}

impl<'sorters> SorterManager<'sorters> {
  pub const fn new(sorters: &'sorters [Sorter]) -> Self {
    Self { sorters }
  }

  pub fn sort(&self, nodes: &mut [FSNodeRes]) {
    for sorter in self.sorters {
      nodes.sort_by(|n1, n2| sorter.sort(n1, n2));
    }
    // TODO: Here is an alternative method, find a way to benchmark it
    // nodes.sort_by(|n1, n2| {
    //   self
    //     .sorters
    //     .iter()
    //     .map(|s| s.sort(n1, n2))
    //     .find(|o| *o != cmp::Ordering::Equal)
    //     .unwrap_or(cmp::Ordering::Equal)
    // });
  }
}
