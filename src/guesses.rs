use std::fmt::Display;

use crate::Word;
use crate::WORD_LENGTH;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Guess {
  Correct(char),
  WrongPosition(char),
  NotIncluded(char),
  Unset,
}

impl Default for Guess {
  fn default() -> Self {
    Guess::Unset
  }
}

impl Display for Guess {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let c: char = (*self).into();
    write!(f, "{}", c)
  }
}

impl From<Guess> for char {
  fn from(val: Guess) -> Self {
    val.to_char()
  }
}

impl Guess {
  fn to_char(self) -> char {
    match self {
      Guess::Correct(c) | Guess::NotIncluded(c) | Guess::WrongPosition(c) => c,
      _ => ' ',
    }
  }
  pub fn render_ansi(self) -> String {
    format!("\u{001b}[37;{};1m {} ", self.ansi_color(), self.to_char())
  }

  fn ansi_color(self) -> usize {
    match self {
      Self::Correct(_) => 45,
      Self::NotIncluded(_) => 40,
      Self::WrongPosition(_) => 41,
      Self::Unset => 49,
    }
  }

  pub fn discord_emoji(self) -> &'static str {
    match self {
      Self::Correct(_) => ":green_square:",
      Self::NotIncluded(_) => ":black_large_square:",
      Self::WrongPosition(_) => ":orange_square:",
      Self::Unset => ":black_large_square:",
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct WordGuess {
  chars: [Guess; WORD_LENGTH],
}

impl WordGuess {
  pub fn new() -> Self {
    Self {
      chars: [Guess::default(); WORD_LENGTH],
    }
  }

  pub fn from_guess(wordle: &Word, guess_chars: &Word) -> Self {
    let mut guesses = Self::new();
    for (i, c) in guess_chars.iter().enumerate() {
      if wordle[i] == *c {
        guesses.chars[i] = Guess::Correct(*c);
      } else if wordle.contains(c) {
        guesses.chars[i] = Guess::WrongPosition(*c);
      } else {
        guesses.chars[i] = Guess::NotIncluded(*c);
      }
    }
    guesses
  }

  pub fn matches(&self) -> bool {
    self.chars.iter().all(|g| matches!(g, Guess::Correct(_)))
  }

  pub fn render_ansi(self) -> String {
    self.chars.into_iter().map(|c| c.render_ansi()).collect()
  }

  pub fn as_discord_emojis(self) -> String {
    self.chars.into_iter().map(|c| c.discord_emoji()).collect()
  }

  pub fn unset(&self) -> bool {
    self.chars.iter().all(|c| c == &Guess::Unset)
  }
}

impl Default for WordGuess {
  fn default() -> Self {
    Self::new()
  }
}

impl Display for WordGuess {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let guess = self
      .chars
      .iter()
      .map(|c| format!("{c}"))
      .collect::<Vec<String>>()
      .join("|");

    write!(f, "{guess}")
  }
}
