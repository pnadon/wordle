use std::error::Error;

use anyhow::anyhow;
use chrono::{Local, NaiveDateTime};

// 19025 days since epoch on Feb 1, 2022. Wordle today was "those", which is on the 228th row (1-based).
// Thus, should be the time since date +%s
const WORDLE_START_DATE: &str = "2021-06-19 0:0:0";

pub fn days_since_start() -> Result<usize, Box<dyn Error>> {
  let start = NaiveDateTime::parse_from_str(WORDLE_START_DATE, "%Y-%m-%d %H:%M:%S")?;
  let now = Local::now().naive_local();
  let diff = (now - start).num_days();
  if diff < 0 {
    return Err(
      anyhow!(
        "Diff between now ({}) and {} cannot be negative",
        now,
        start
      )
      .into(),
    );
  }
  Ok(diff as usize)
}
