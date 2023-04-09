// TODO: rename lifetime
use crate::fs_node::FSNodeRes;

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
