/* Modules */
mod functions;
/* Crate imports */
use crate::fs_node::{FSNode, FSNodeRes};

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

type FilteringMethod = Box<dyn Fn(&FSNode) -> bool>;
impl Filter {
  pub fn get_filtering_method(self) -> FilteringMethod {
    match self {
      Self::Hidden => Box::new(functions::is_hidden),
      Self::ExtensionE(exts) => {
        Box::new(move |n| functions::filter_ext_exc(n, &exts))
      },
      Self::ExtensionI(exts) => {
        Box::new(move |n| functions::filter_ext_inc(n, &exts))
      },
      Self::Files => Box::new(functions::is_file),
      Self::SymLinks => Box::new(functions::is_symlink),
      #[allow(clippy::unreachable)]
      Self::Error => unreachable!("Cannot be reached as it is tested before"),
    }
  }
}

pub struct FilterManager {
  filtering_methods: Vec<FilteringMethod>,
  keep_errors: bool,
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
    let mut keep_errors = false;
    let filtering_methods = filters
      .into_iter()
      .filter(|f| {
        keep_errors |= matches!(f, &Filter::Error);
        !matches!(f, &Filter::Error)
      })
      .map(Filter::get_filtering_method)
      .collect::<Vec<FilteringMethod>>();

    Self {
      filtering_methods,
      keep_errors,
    }
  }

  pub fn filter(&self, node: &FSNodeRes) -> bool {
    // TODO: can we remove
    // `.as_ref()` 
    // `!` (not very readable)
    !node.as_ref().map_or(self.keep_errors, |n| {
      self.filtering_methods.iter().any(|f| f(n))
    })
  }
}
