use std::error::Error;
use wordle::backends::{cli, discord};

const RUN_CLI: bool = false;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  if RUN_CLI {
    cli::run()
  } else {
    discord::run().await
  }
}
