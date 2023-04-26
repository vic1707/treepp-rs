/* Modules */
mod methods;
/* Built in imports */
use core::cmp;
/* Crate imports */
use crate::fs_node::FSNodeRes;

#[derive(Clone, clap::ValueEnum)]
pub enum Sorter {
  Name,
  Size,
  Modified,
  // FileFolder, // TODO: implement
  // FolderFile, // TODO: implement
  Extension,
}

type SortingMethod = fn(&FSNodeRes, &FSNodeRes) -> cmp::Ordering;
impl Sorter {
  pub fn get_sorting_method(self) -> SortingMethod {
    match self {
      Self::Name => methods::name,
      Self::Size => methods::size,
      Self::Extension => methods::extension,
      Self::Modified => methods::modified_date,
    }
  }
}

pub struct SorterManager {
  sorting_methods: Vec<SortingMethod>,
}

impl SorterManager {
  pub fn new(sorters: Vec<Sorter>) -> Self {
    let sorting_methods = sorters
      .into_iter()
      .map(Sorter::get_sorting_method)
      .collect::<Vec<SortingMethod>>();

    Self { sorting_methods }
  }

  pub fn sort(&self, nodes: &mut [FSNodeRes]) {
    for method in &self.sorting_methods {
      nodes.sort_by(method);
    }
  }
}
