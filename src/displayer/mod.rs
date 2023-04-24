use crate::{formatter::FormatterT, FSNode, FSNodeRes};

#[derive(Clone, clap::ValueEnum)]
pub enum Mode {
  Fancy,  // "─", "│", "├", "└"
  Spaces, // " ", " ", " ", " "
  #[clap(skip)]
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
  pub fn new(mode: &Mode, tab_size: usize) -> Self {
    let [h, v, t, b] = mode.get();
    // minus 2 to take the first char & leading space into account
    let padding = tab_size - 2;
    Self([
      format!("{t}{} ", h.to_string().repeat(padding)),
      format!("{v}{} ", " ".repeat(padding)),
      format!("{b}{} ", h.to_string().repeat(padding)),
      " ".repeat(tab_size),
    ])
  }

  pub fn display(
    &self,
    node: &FSNodeRes,
    prefixes: [&str; 2],
    formatter: &impl FormatterT,
  ) {
    println!("{}{}", prefixes[0], formatter.format(node));
    let Ok(FSNode::Dir(ref dir)) = *node else { return; };

    let new_prefixes = self.0.clone().map(|p| format!("{}{p}", prefixes[1]));

    let num_entries = dir.entries().len();
    dir.entries().iter().enumerate().for_each(|(i, n)| {
      if i == num_entries - 1 {
        self.display(n, [&new_prefixes[2], &new_prefixes[3]], formatter);
      } else {
        self.display(n, [&new_prefixes[0], &new_prefixes[1]], formatter);
      }
    });
  }
}
