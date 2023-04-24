use crate::fs_node::{Dir, FSNodeError, File, Symlink};
use crate::{FSNode, FSNodeRes};

pub mod name_only;

pub trait FormatterT {
  fn format_dir(dir: &Dir) -> String;
  fn format_file(file: &File) -> String;
  fn format_symlink(symlink: &Symlink) -> String;
  fn format_err(err: &FSNodeError) -> String {
    format!("{err}")
  }

  fn format(node_: &FSNodeRes) -> String {
    match *node_ {
      Ok(ref node) => match *node {
        FSNode::Dir(ref dir) => Self::format_dir(dir),
        FSNode::File(ref file) => Self::format_file(file),
        FSNode::Symlink(ref symlink) => Self::format_symlink(symlink),
      },
      Err(ref err) => Self::format_err(err),
    }
  }
}
