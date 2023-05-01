/* Modules */
pub mod name_only;
/* Crate imports */
use crate::fs_node::{
  dir::Dir, file::File, symlink::Symlink, FSNode, FSNodeError, FSNodeRes,
};

#[derive(Clone, clap::ValueEnum)]
pub enum Formatter {
  NameOnly,
}

impl Formatter {
  pub const fn get(&self) -> &impl FormatterT {
    match *self {
      Self::NameOnly => &name_only::NameOnly,
    }
  }
}

pub trait FormatterT {
  fn format_dir(&self, dir: &Dir) -> String;
  fn format_file(&self, file: &File) -> String;
  fn format_symlink(&self, symlink: &Symlink) -> String;
  fn format_err(&self, err: &FSNodeError) -> String {
    format!("{err}")
  }

  fn format(&self, node_: &FSNodeRes) -> String {
    match *node_ {
      Ok(ref node) => match *node {
        FSNode::Dir(ref dir) => self.format_dir(dir),
        FSNode::File(ref file) => self.format_file(file),
        FSNode::Symlink(ref symlink) => self.format_symlink(symlink),
      },
      Err(ref err) => self.format_err(err),
    }
  }
}
