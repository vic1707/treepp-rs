// TODO: rename lifetime
use crate::fs_node::{FSNode, FSNodeRes};

mod functions;

#[derive(Debug)]
pub enum Filter<'a> {
  Hidden,
  Extension(&'a [&'a str]),
  Files,
  SymLinks,
  Error,
}

impl Filter<'_> {
  pub fn filter(&self, node_: &FSNodeRes) -> bool {
    // TODO: fix, this is not good for the Error variant
    if let Ok(ref node) = *node_ {
      return match *self {
        Self::Hidden => functions::is_hidden(node),
        Self::Extension(exts) => functions::filter_ext_exc(node, exts),
        Self::Files => functions::is_file(node),
        Self::SymLinks => functions::is_symlink(node),
        Self::Error => false,
      };
    }
    false
  }
}

pub struct FilterManager<'a> {
  filters: &'a [Filter<'a>],
}

impl<'a> FilterManager<'a> {
  pub const fn new(filters: &'a [Filter<'a>]) -> Self {
    Self { filters }
  }

  pub fn apply(&self, nodes: &mut Vec<FSNodeRes>) {
    nodes.retain_mut(|node| self.filter_node(node));
  }

  fn filter_node(&self, node_: &mut FSNodeRes) -> bool {
    if let Ok(FSNode::Directory(ref mut node)) = *node_ {
      node.entries_mut().retain_mut(|n| self.filter_node(n));
    }
    // TODO: check logic and rewrite
    !self.filters.iter().any(|filter| filter.filter(node_))
  }
}
