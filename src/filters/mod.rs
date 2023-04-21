use crate::fs_node::FSNodeRes;

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
  pub fn filter(&self, node: &FSNodeRes) -> bool {
    node
      .as_ref()
      .map_or(matches!(self, &Self::Error), |n| match *self {
        Self::Hidden => functions::is_hidden(n),
        Self::Extension(ref exts) => functions::filter_ext_exc(n, exts),
        Self::Files => functions::is_file(n),
        Self::SymLinks => functions::is_symlink(n),
        Self::Error => false,
      })
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
    !self.filters.iter().any(|f| f.filter(node))
  }
}
