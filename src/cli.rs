use clap::{command, Parser};
use std::{fs, io, path::PathBuf};

use crate::displayer::Mode;
use crate::filters::Filter;
use crate::sorters::Sorter;

/// Rust implementation of the tree-plus-plus command line tool.
#[derive(Parser)]
#[command(author, version, about, propagate_version = true)]
pub struct Options {
  /// Print all files and directories, including hidden ones.
  #[arg(short, long)]
  pub all: bool,

  /// Positional argument for the paths to list
  #[arg(default_value = ".", value_parser = canonicalize_dir)]
  pub paths: Vec<PathBuf>,

  /// Sorters to apply to the list of files and directories.
  #[arg(short, long = "sorter", value_name = "sorter", value_enum)]
  pub sorters: Vec<Sorter>,

  /// Filters to apply to the list of files and directories.
  #[arg(short, long = "filter", value_name = "filter", value_enum)]
  pub filters: Vec<Filter>,

  /// Extensions to filter out.
  #[arg(long = "filter-out-extension", value_name = "ext")]
  pub exts_e: Vec<String>,

  /// Extensions to filter in.
  #[arg(long = "filter-in-extension", value_name = "ext")]
  pub exts_i: Vec<String>,

  /// Displaying mode.
  #[arg(short, long, value_name = "mode", value_enum, default_value = "fancy")]
  pub mode: Mode,

  /// Tab size
  #[arg(short, long, default_value = "4")]
  pub tab_size: usize,
}

fn canonicalize_dir(p: &str) -> io::Result<PathBuf> {
  match fs::canonicalize(p) {
    Ok(path) if path.is_dir() => Ok(path),
    _ => Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("{p} is not a directory"),
    )),
  }
}
