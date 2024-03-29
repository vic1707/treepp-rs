/* Clippy config */
#![warn(
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
    // clippy::cargo                          // disabled for now
  )]
#![allow(
    clippy::integer_arithmetic,               // used to calculate size of directory
    clippy::arithmetic_side_effects,          // used to calculate size of directory
    clippy::blanket_clippy_restriction_lints, // allow clippy::restriction to be used globally
    clippy::implicit_return,
    clippy::missing_docs_in_private_items,    // allowed for now
    clippy::missing_trait_methods,            // warns when not overriding a default method
    clippy::mod_module_files,
    clippy::panic,                            // allowed for now
    clippy::print_stdout,                     // allowed for now
    clippy::print_stderr,                     // allowed for now
    clippy::question_mark_used,
    dead_code,                                // allowed for now
  )]

/* Modules */
mod cli;
mod displayer;
mod filters;
mod formatters;
mod fs_node;
mod sorters;
/* External crates imports */
use clap::Parser;
/* Crate imports */
use cli::Options;
use displayer::Displayer;
use filters::FilterManager;
use fs_node::{FSNode, FSNodeRes};
use sorters::SorterManager;

fn main() {
  let opts = Options::parse();
  let displayer = Displayer::new(&opts.mode, opts.tab_size);
  let sorter_manager = SorterManager::new(opts.sorters);
  let filter_manager =
    FilterManager::new(opts.filters, opts.all, opts.exts_e, opts.exts_i);
  let formatter = opts.formatter.get();

  opts
    .paths
    .iter()
    .map(|n| FSNode::build(n, &filter_manager, &sorter_manager))
    .filter(|n| filter_manager.filter(n))
    .for_each(|ref n| {
      displayer.display(n, ["", ""], formatter);
      println!();
    });
}
