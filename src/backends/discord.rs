use std::collections::HashMap;
use std::{env, error::Error};

use serenity::{
  async_trait,
  model::{channel::Message, prelude::*},
  prelude::*,
};

use crate::board::{Board, State};
use crate::date_to_word::days_since_start;
use crate::word_bank::WordBank;
use crate::{discord_err_msg, parse_word_err, print_discord_err_msg, Word, DAY_WORDS, GUESS_WORDS};

struct PersistentData;

impl TypeMapKey for PersistentData {
  type Value = HashMap<u64, Board>;
}

struct Handler {
  word_bank: WordBank,
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    if !msg.content.starts_with('?') {
      return;
    }

    let id = *msg.author.id.as_u64();
    let mut data = ctx.data.write().await;
    let boards: &mut HashMap<u64, Board> = data.get_mut::<PersistentData>().unwrap();

    let word_of_the_day = self
      .word_bank
      .get_word(days_since_start().unwrap())
      .unwrap();

    match boards.get_mut(&id) {
      None => {
        boards.insert(id, Board::new(*word_of_the_day));
      }
      Some(board) => {
        if word_of_the_day != board.correct_word() {
          if let Err(e) = send_msg(
            &ctx,
            &msg,
            &format!(
              "Here's your score for today:\n{}",
              board.as_discord_emojis()
            ),
          )
          .await
          {
            print_discord_err_msg(e);
          };
          *board = Board::new(*word_of_the_day);
        }
        match board.state() {
          State::Lost => {
            if let Err(e) = send_msg(
              &ctx,
              &msg,
              "You've already played today, try again tomorrow!",
            )
            .await
            {
              print_discord_err_msg(e);
            }
          }
          State::Won => {
            if let Err(e) = send_msg(
              &ctx,
              &msg,
              &format!(
                "Here's your score for today:\n{}",
                board.as_discord_emojis()
              ),
            )
            .await
            {
              println!("{}", e);
            }
          }
          _ => {
            if let Err(e) = handle_move(&ctx, msg, board, &self.word_bank).await {
              println!("{:?}", e);
            }
          }
        }
      }
    }
  }

  async fn ready(&self, ctx: Context, ready: Ready) {
    ctx
      .set_activity(Activity::playing("Type \"?help\" to get started!"))
      .await;
    println!("{} is connected!", ready.user.name);
  }
}

async fn handle_move(
  ctx: &Context,
  msg: Message,
  board: &mut Board,
  word_bank: &WordBank,
) -> Result<(), String> {
  let resp_msg = play_move(&msg, board, word_bank)?;
  send_msg(ctx, &msg, &resp_msg).await?;

  match board.state() {
    State::Lost =>
      send_msg(ctx, &msg, "You lost! Try again tomorrow!").await,
    State::Playing => Ok(()),
    State::Won =>
      send_msg(
        ctx,
        &msg,
        &format!("You won!\n{}", board.as_discord_emojis()),
      ).await
  }
}

fn play_move(msg: &Message, board: &mut Board, word_bank: &WordBank) -> Result<String, String> {
  let guess: Word = msg
    .content
    .trim()
    .chars()
    .skip(1)
    .collect::<Vec<char>>()
    .try_into()
    .map_err(|_| format!("{:?}", parse_word_err(&msg.content)))?;

  if word_bank.is_valid_guess(&guess) {
    board.guess(&guess);
    Ok(format!("```ansi\n{}\n```", board.render_ansi()))
  } else {
    Ok("Invalid input, try again".to_string())
  }
}

async fn send_msg(ctx: &Context, msg: &Message, body: &str) -> Result<(), String> {
  msg
    .channel_id
    .say(&ctx.http, body)
    .await
    .map_err(discord_err_msg)?;
  Ok(())
}

pub async fn run() -> Result<(), Box<dyn Error>> {
  let token = env::var("WORDLE_BOT_TOKEN").expect("Could not find token.");
  let word_bank = WordBank::from_raw(DAY_WORDS, GUESS_WORDS)?;
  let mut client = Client::builder(&token)
    .event_handler(Handler { word_bank })
    .await?;

  client
    .data
    .write()
    .await
    .insert::<PersistentData>(HashMap::new());

  client.start().await?;
  Ok(())
}
