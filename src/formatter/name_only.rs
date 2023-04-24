use super::Formatter;
use crate::fs_node::{Dir, File, Symlink};

pub struct NameOnly;

impl Formatter for NameOnly {
  fn format_dir(dir: &Dir) -> String {
    dir.path().file_name().unwrap().to_str().unwrap().to_owned()
  }

  fn format_file(file: &File) -> String {
    file
      .path()
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_owned()
  }

  fn format_symlink(symlink: &Symlink) -> String {
    symlink
      .path()
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_owned()
  }
}