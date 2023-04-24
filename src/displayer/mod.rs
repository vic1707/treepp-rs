use crate::{FSNode, FSNodeRes};

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

  pub fn display(&self, node_: &FSNodeRes, prefixes: [&str; 2]) {
    match *node_ {
      Ok(ref node) => {
        // TODO: impl Display for FSNode
        // println!("{}{node}", prefixes[0]);

        let FSNode::Dir(ref dir) = *node else { return; };

        let new_prefixes =
          self.0.clone().map(|p| format!("{}{p}", prefixes[1]));

        let num_entries = dir.entries().len();
        dir.entries().iter().enumerate().for_each(|(i, n)| {
          if i == num_entries - 1 {
            self.display(n, [&new_prefixes[2], &new_prefixes[3]]);
          } else {
            self.display(n, [&new_prefixes[0], &new_prefixes[1]]);
          }
        });
      },
      Err(ref err) => println!("{}{err}", prefixes[0]),
    }
  }
}
