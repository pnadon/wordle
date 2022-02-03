use anyhow::anyhow;
use std::{collections::HashSet, error::Error};

use crate::{parse_word_err, Word};

pub struct WordBank {
  day_words: Vec<Word>,
  guesses: HashSet<Word>,
}

impl WordBank {
  pub fn from_raw(day_words: &str, guesses: &str) -> Result<Self, Box<dyn Error>> {
    Ok(Self {
      day_words: str_to_words(day_words)?,
      guesses: str_to_words(&(guesses.to_owned() + day_words))?,
    })
  }

  pub fn get_word(&self, index: usize) -> Option<&Word> {
    self.day_words.get(index)
  }

  pub fn is_valid_guess(&self, guess: &Word) -> bool {
    self.guesses.contains(guess)
  }
}

fn str_to_words<T>(words: &str) -> Result<T, Box<dyn Error>>
where
  T: FromIterator<Word>,
{
  words
    .split('\n')
    .map(|word| word.trim())
    .filter(|w| !w.is_empty())
    .map(|word| {
      let chars: Word = word
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .map_err(|_| parse_word_err(words))?;

      Ok(chars)
    })
    .collect()
}
