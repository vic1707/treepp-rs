/* Crate imports */
use super::FormatterT;
use crate::fs_node::{dir::Dir, file::File, symlink::Symlink};

pub struct NameOnly;

impl FormatterT for NameOnly {
  fn format_dir(&self, dir: &Dir) -> String {
    dir.filename.clone()
  }

  fn format_file(&self, file: &File) -> String {
    file.filename.clone()
  }

  fn format_symlink(&self, symlink: &Symlink) -> String {
    format!(
      "{} -> {}",
      symlink.filename.clone(),
      symlink.target_filename.clone()
    )
  }
}
