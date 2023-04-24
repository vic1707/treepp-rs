use crate::fs_node::{Dir, FSNodeError, File, Symlink};
use crate::{FSNode, FSNodeRes};

pub mod name_only;

pub enum Mode {
  Fancy,  // "─", "│", "├", "└"
  Spaces, // " ", " ", " ", " "
  Custom(char, char, char, char),
}

impl Mode {
  pub const fn get(&self) -> [char; 4] {
    match *self {
      Self::Fancy => ['\u{2500}', '\u{2502}', '\u{251c}', '\u{2514}'],
      Self::Spaces => [' ', ' ', ' ', ' '],
      Self::Custom(h, v, t, b) => [h, v, t, b],
    }
  }
}

// mode and tab size
pub struct Displayer([String; 4]);

impl Displayer {
  pub fn new(mode: &Mode, depth_: usize) -> Self {
    let [h, v, t, b] = mode.get();
    // minus 2 to take the first char & leading space into account
    let depth = depth_ - 2;
    Self([
      format!("{t}{} ", h.to_string().repeat(depth)),
      format!("{v}{} ", " ".repeat(depth)),
      format!("{b}{} ", h.to_string().repeat(depth)),
      " ".repeat(depth_),
    ])
  }

  pub fn display<T: Formatter>(&self, node: &FSNodeRes, prefixes: [&str; 2]) {
    println!("{}{}", prefixes[0], T::format(node));
    let Ok(FSNode::Dir(ref dir)) = *node else { return; };

    let new_prefixes = self.0.clone().map(|p| format!("{}{p}", prefixes[1]));

    let num_entries = dir.entries().len();
    dir.entries().iter().enumerate().for_each(|(i, n)| {
      if i == num_entries - 1 {
        self.display::<T>(n, [&new_prefixes[2], &new_prefixes[3]]);
      } else {
        self.display::<T>(n, [&new_prefixes[0], &new_prefixes[1]]);
      }
    });
  }
}

pub trait Formatter {
  fn format_dir(dir: &Dir) -> String;
  fn format_file(file: &File) -> String;
  fn format_symlink(symlink: &Symlink) -> String;
  fn format_err(err: &FSNodeError) -> String {
    format!("{err}")
  }

  fn format(node_: &FSNodeRes) -> String {
    match *node_ {
      Ok(ref node) => match *node {
        FSNode::Dir(ref dir) => Self::format_dir(dir),
        FSNode::File(ref file) => Self::format_file(file),
        FSNode::Symlink(ref symlink) => Self::format_symlink(symlink),
      },
      Err(ref err) => Self::format_err(err),
    }
  }
}
