// TODO: rename lifetime
use crate::fs_node::FSNodeRes;

mod methods;

#[derive(Debug)]
pub enum Sorter {
  Name,
  Size,
  Modified,
  FileFolder,
  FolderFile,
  Extension,
}

impl Sorter {
  pub fn sort(&self, nodes: &mut [FSNodeRes]) {
    match *self {
      Self::Name => nodes.sort_by_key(methods::name),
      Self::Size => nodes.sort_by_key(methods::size),
      Self::Extension => nodes.sort_by(methods::extension),
      _ => todo!(),
    }
  }
}

pub struct SorterManager<'a> {
  sorters: &'a [Sorter],
}

impl <'a> SorterManager<'a> {
  pub const fn new(sorters: &'a [Sorter]) -> Self {
    Self { sorters }
  }

  pub fn apply(&self, nodes: &mut [FSNodeRes]) {
    for sorter in self.sorters {
      sorter.sort(nodes);
    }
  }
}
