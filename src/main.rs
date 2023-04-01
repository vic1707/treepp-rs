// Clippy config
#![warn(
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo                          // disabled for now
  )]
#![allow(
    clippy::blanket_clippy_restriction_lints, // allowed for now
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,    // allowed for now
    clippy::mod_module_files,
    clippy::print_stdout,                     // allowed for now
    clippy::print_stderr,                     // allowed for now
    clippy::todo,                             // allowed for now
    clippy::use_debug,                        // allowed for now
    dead_code,                                // allowed for now
  )]

/* Modules */
mod cli;
mod fs_node;
/* Use */
use clap::Parser;
use cli::Options;
use fs_node::FSNode;

fn main() {
  let opts = Options::parse();

  println!("{opts:#?}");

  for path in opts.paths {
    let fs_node = FSNode::build(path);
    println!("{fs_node:#?}");
  }
}
