use std::fmt::Display;

use crate::{guesses::WordGuess, render_ansi_newline, Word, NUM_GUESSES};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
  Won,
  Lost,
  Playing,
}

impl Display for State {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

pub struct Board {
  word_guesses: [WordGuess; NUM_GUESSES],
  state: State,
  correct_word: Word,
  cursor: usize,
}

impl Board {
  pub fn new(correct_word: Word) -> Self {
    Self {
      word_guesses: [WordGuess::new(); NUM_GUESSES],
      state: State::Playing,
      correct_word,
      cursor: 0,
    }
  }

  pub fn guess(&mut self, word: &Word) -> State {
    self.word_guesses[self.cursor] = WordGuess::from_guess(&self.correct_word, word);

    self.state = if self.word_guesses[self.cursor].matches() {
      State::Won
    } else if self.cursor == NUM_GUESSES - 1 {
      State::Lost
    } else {
      self.cursor += 1;
      State::Playing
    };
    self.state
  }

  pub fn render_ansi(&self) -> String {
    self
      .word_guesses
      .iter()
      .map(|word| word.render_ansi() + render_ansi_newline())
      .collect()
  }

  pub fn as_discord_emojis(&self) -> String {
    self
      .word_guesses
      .iter()
      .filter(|g| !g.unset())
      .map(|word| word.as_discord_emojis() + "\n")
      .collect::<String>()
      + &format!(
        "{} / {}",
        self.word_guesses.iter().filter(|g| !g.unset()).count(),
        NUM_GUESSES
      )
  }

  pub fn state(&self) -> State {
    self.state
  }

  pub fn correct_word(&self) -> &Word {
    &self.correct_word
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let res = self
      .word_guesses
      .iter()
      .map(|guess| format!("{guess}\n"))
      .collect::<String>();
    write!(f, "{res}")
  }
}
