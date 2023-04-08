use clap::{command, Parser};
use std::{fs, io, path::PathBuf};

/// Rust implementation of the tree-plus-plus command line tool.
#[derive(Parser, Debug)]
#[command(author, version, about, propagate_version = true)]
pub struct Options {
  /// Print all files and directories, including hidden ones.
  #[arg(short, long)]
  pub all: bool,

  /// Positional argument for the paths to list
  #[arg(default_value = ".", value_parser = canonicalize_dir)]
  pub paths: Vec<PathBuf>,
}

fn canonicalize_dir(path: &str) -> io::Result<PathBuf> {
  let p = fs::canonicalize(path)?;
  // if path isn't a dir we return an error
  if !p.is_dir() {
    return Err(io::Error::new(
      io::ErrorKind::InvalidInput,
      format!("{path} is not a directory"),
    ));
  }
  Ok(p)
}
