use std::{error::Error, io};

use crate::{
  board::{Board, State},
  date_to_word::days_since_start,
  word_bank::WordBank,
  DAY_WORDS, GUESS_WORDS,
};
use crate::{parse_word_err, Word};

pub fn run() -> Result<(), Box<dyn Error>> {
  let word_bank = WordBank::from_raw(DAY_WORDS, GUESS_WORDS)?;
  let mut _last_date_won = None;
  loop {
    let word_of_the_day = word_bank.get_word(days_since_start()?).unwrap();
    let board = Board::new(*word_of_the_day);
    if handle_user_input(board, &word_bank)? == State::Won {
      _last_date_won = Some(chrono::Local::now());
    }
  }
}

fn handle_user_input(board: Board, word_bank: &WordBank) -> Result<State, Box<dyn Error>> {
  let mut board = board;
  let mut buf = String::new();

  while board.state() == State::Playing {
    buf.clear();
    println!("Enter your guess");
    io::stdin().read_line(&mut buf)?;
    let guess: Word = buf
      .trim()
      .chars()
      .collect::<Vec<char>>()
      .try_into()
      .map_err(|_| parse_word_err(&buf))?;

    if word_bank.is_valid_guess(&guess) {
      board.guess(&guess);
      println!("{}", board.render_ansi());
    } else {
      println!("invalid input, try again");
    };
  }
  println!("Result: {}", board.state());
  Ok(board.state())
}
