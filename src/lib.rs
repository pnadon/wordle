use std::fmt::Debug;

use anyhow::anyhow;

const NUM_GUESSES: usize = 6;
const WORD_LENGTH: usize = 5;
pub const DAY_WORDS: &str = include_str!("words/wordles.txt");
pub const GUESS_WORDS: &str = include_str!("words/guesses.txt");

pub fn parse_word_err(word: &str) -> anyhow::Error {
  if word.len() > 10 {
    anyhow!(
      "Could not fit word into 5 character array: {}...",
      &word[..10]
    )
  } else {
    anyhow!("Could not fit word into 5 character array: {word}")
  }
}

pub fn discord_err_msg(msg: impl Debug) -> String {
  format!("Error sending message: {:?}", msg)
}

pub fn print_discord_err_msg(msg: impl Debug) {
  println!("{}", discord_err_msg(msg))
}

pub fn render_ansi_newline() -> &'static str {
  "\u{001b}[0m\n"
}

pub type Word = [char; WORD_LENGTH];

pub mod backends;
pub mod board;
pub mod date_to_word;
pub mod guesses;
pub mod word_bank;
