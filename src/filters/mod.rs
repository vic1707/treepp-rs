use crate::fs_node::{FSNode, FSNodeRes};

mod functions;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Filter {
  #[clap(skip)]
  Hidden,
  #[clap(skip)]
  Extension(Vec<String>),
  Files,
  SymLinks,
  Error,
}

impl Filter {
  pub fn filter(&self, node_: &FSNodeRes) -> bool {
    // TODO: fix, this is not good for the Error variant
    if let Ok(ref node) = *node_ {
      return match *self {
        Self::Hidden => functions::is_hidden(node),
        Self::Extension(ref exts) => functions::filter_ext_exc(node, exts),
        Self::Files => functions::is_file(node),
        Self::SymLinks => functions::is_symlink(node),
        Self::Error => false,
      };
    }
    false
  }
}

pub struct FilterManager {
  filters: Vec<Filter>,
}

impl FilterManager {
  pub fn new(
    mut filters: Vec<Filter>,
    hidden: bool,
    exts: Vec<String>,
  ) -> Self {
    if !hidden {
      filters.push(Filter::Hidden);
    }
    if !exts.is_empty() {
      filters.push(Filter::Extension(exts));
    }
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
