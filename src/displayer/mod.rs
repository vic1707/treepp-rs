pub enum Mode {
  Fancy,  // "─", "│", "├", "└"
  Spaces, // " ", " ", " ", " "
  Custom(&'static str, &'static str, &'static str, &'static str),
}

impl Mode {
  pub const fn get(&self) -> [&str; 4] {
    match *self {
      Self::Fancy => ["\u{2500}", "\u{2502}", "\u{251c}", "\u{2514}"],
      Self::Spaces => [" ", " ", " ", " "],
      Self::Custom(h, v, t, b) => [h, v, t, b],
    }
  }
}
