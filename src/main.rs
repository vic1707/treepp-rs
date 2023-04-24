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
    clippy::missing_trait_methods,            // warns when not overriding a default method
    clippy::mod_module_files,
    clippy::panic,                            // allowed for now
    clippy::print_stdout,                     // allowed for now
    clippy::print_stderr,                     // allowed for now
    clippy::pub_use,
    clippy::question_mark_used,
    clippy::todo,                             // allowed for now
    clippy::use_debug,                        // allowed for now
    clippy::wildcard_enum_match_arm,          // allowed for now
    dead_code,                                // allowed for now
  )]

/* Modules */
mod cli;
mod displayer;
mod filters;
mod formatter;
mod fs_node;
mod sorters;
/* Use */
use clap::Parser;
use cli::Options;
use filters::FilterManager;
use formatter::name_only::NameOnly;
use fs_node::{FSNode, FSNodeRes};
use sorters::SorterManager;

fn main() {
  let opts = Options::parse();
  let displayer = displayer::Displayer::new(&opts.mode, opts.tab_size);
  let sorter_manager = SorterManager::new(opts.sorters);
  let filter_manager =
    FilterManager::new(opts.filters, opts.all, opts.exts_e, opts.exts_i);

  opts
    .paths
    .iter()
    .map(|n| FSNode::build(n, &filter_manager, &sorter_manager))
    .filter(|n| filter_manager.filter(n))
    .for_each(|ref n| displayer.display(n, ["", ""], &NameOnly {}));
}
