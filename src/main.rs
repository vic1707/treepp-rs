// Clippy config
#![warn(
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo                          // disabled for now
  )]
#![allow(
    clippy::integer_arithmetic,               // used to calculate size of directory
    clippy::arithmetic_side_effects,          // used to calculate size of directory
    clippy::blanket_clippy_restriction_lints, // allowed for now
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,    // allowed for now
    clippy::mod_module_files,
    clippy::print_stdout,                     // allowed for now
    clippy::print_stderr,                     // allowed for now
    clippy::pub_use,
    clippy::todo,                             // allowed for now
    clippy::use_debug,                        // allowed for now
    dead_code,                                // allowed for now
  )]

/* Modules */
mod cli;
mod filters;
mod fs_node;
mod sorters;
/* Use */
use clap::Parser;
use cli::Options;
use filters::{Filter, FilterManager};
use fs_node::{FSNode, FSNodeRes};
use sorters::{Sorter, SorterManager};

fn main() {
  let opts = Options::parse();

  let mut nodes = opts
    .paths
    .iter()
    .map(FSNode::build)
    .collect::<Vec<FSNodeRes>>();

  // Exemple of use of FilterManager
  FilterManager::new(&[
    Filter::Hidden,
    Filter::Extension(&["rs", "toml"]),
    Filter::Files,
  ])
  .apply(&mut nodes);

  // Exemple of use of SorterManager
  SorterManager::new(&[
    Sorter::Name,
    Sorter::Extension
  ])
  .apply(&mut nodes);

  println!("{nodes:#?}");
}
