use crate::fs_node::{FSNode, FSNodeRes};

mod functions;

// TODO: Extension variant is exclusive, make an inclusive variant
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

  pub fn filter(&self, node: &FSNodeRes) -> bool {
    // TODO: check logic and maybe rewrite
    self.filters.iter().any(|f| !f.filter(node))
  }
}
