use clap::{command, Parser};
use std::path::PathBuf;

/// Rust implementation of the tree-plus-plus command line tool.
#[derive(Parser, Debug)]
#[command(author, version, about, propagate_version = true)]
pub struct Options {
  /// Print all files and directories, including hidden ones.
  #[arg(short, long)]
  pub all: bool,

  /// Positional argument for the paths to list
  #[arg(default_value = ".")]
  pub paths: Vec<PathBuf>,
}
