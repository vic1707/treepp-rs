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
