/* External crates imports */
use clap::{command, Parser};
/* Built in imports */
use std::{fs, io, path::PathBuf};
/* Crate imports */
use crate::displayer::Mode;
use crate::filters::Filter;
use crate::formatters::Formatter;
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
  #[arg(short, long = "sorter", value_enum)]
  pub sorters: Vec<Sorter>,

  /// Filters to apply to the list of files and directories.
  #[arg(short, long = "filter", value_enum)]
  pub filters: Vec<Filter>,

  /// Extensions to filter out.
  #[arg(long = "filter-out-extension")]
  pub exts_e: Vec<String>,

  /// Extensions to filter in.
  #[arg(long = "filter-in-extension")]
  pub exts_i: Vec<String>,

  /// Displaying mode.
  #[arg(short, long, value_enum, default_value = "fancy")]
  pub mode: Mode,

  /// Tab size
  #[arg(short, long, default_value = "4")]
  pub tab_size: usize,

  /// Formatter to use.
  #[arg(short = 'F', long, value_enum, default_value = "name-only")]
  pub formatter: Formatter,
}

fn canonicalize_dir(p: &str) -> io::Result<PathBuf> {
  let path = fs::canonicalize(p)?;
  if path.exists() {
    Ok(path)
  } else {
    Err(io::Error::new(
      io::ErrorKind::NotFound,
      format!("No such file or directory: '{p}'"),
    ))
  }
}
