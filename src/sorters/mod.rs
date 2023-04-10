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
  pub fn sort(&self, nodes: &mut [FSNodeRes]) {
    match *self {
      Self::Name => nodes.sort_by_key(methods::name),
      Self::Size => nodes.sort_by_key(methods::size),
      Self::Extension => nodes.sort_by(methods::extension),
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

  pub fn apply(&self, nodes: &mut [FSNodeRes]) {
    for sorter in self.sorters {
      sorter.sort(nodes);
    }
  }
}
