#![allow(clippy::unwrap_used)] // as this file is a placeholder/POC
use super::FormatterT;
use crate::fs_node::{Dir, File, Symlink};

pub struct NameOnly;

impl FormatterT for NameOnly {
  fn format_dir(&self, dir: &Dir) -> String {
    dir.path().file_name().unwrap().to_str().unwrap().to_owned()
  }

  fn format_file(&self, file: &File) -> String {
    file
      .path()
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_owned()
  }

  fn format_symlink(&self, symlink: &Symlink) -> String {
    symlink
      .path()
      .file_name()
      .unwrap()
      .to_str()
      .unwrap()
      .to_owned()
  }
}