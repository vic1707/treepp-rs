use crate::fs_node::FSNodeRes;

mod functions;

#[derive(Clone, clap::ValueEnum)]
pub enum Filter {
  #[clap(skip)]
  Hidden,
  #[clap(skip)]
  ExtensionE(Vec<String>),
  #[clap(skip)]
  ExtensionI(Vec<String>),
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
        Self::ExtensionE(ref exts) => functions::filter_ext_exc(n, exts),
        Self::ExtensionI(ref exts) => functions::filter_ext_inc(n, exts),
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
    exts_e: Vec<String>,
    exts_i: Vec<String>,
  ) -> Self {
    if !hidden {
      filters.push(Filter::Hidden);
    }
    if !exts_e.is_empty() {
      filters.push(Filter::ExtensionE(exts_e));
    }
    if !exts_i.is_empty() {
      filters.push(Filter::ExtensionI(exts_i));
    }
    Self { filters }
  }

  pub fn filter(&self, node: &FSNodeRes) -> bool {
    !self.filters.iter().any(|f| f.filter(node))
  }
}
