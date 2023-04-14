use dotenv::dotenv;
use std::env;
use std::{
  io::{self, BufReader},
  error,
  fs::File,
};
use crossterm::{
  terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
  execute,
  event::{EnableMouseCapture, DisableMouseCapture},
};
use tui::{
  Terminal,
  backend::CrosstermBackend,
};
use types::Config;

use crate::requests::health;

pub mod colors;
pub mod enums;
pub mod models;
pub mod requests;
pub mod state;
pub mod types;
pub mod ui;
pub mod utils;

fn load_config() -> io::Result<Config> {
  let path = match env::var("ENV") {
    Ok(env) => {
      if let "DEV" = &*env {
        "config.dev.json"
      } else {
        "config.json"
      }
    }
    Err(_) => "config.json",
  };

  let file = File::open(path)?;
  let reader = BufReader::new(file);
  let data: Config = serde_json::from_reader(reader)?;
  Ok(data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
  dotenv().ok();
  let client = reqwest::Client::new();

  let ping = health::request(&client).await;
  if ping.is_err() {
    println!("{}", String::from("Error connect to server"));
    std::process::exit(1)
  }

  terminal::enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let res = ui::run_app(&mut terminal, &client).await;

  terminal::disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;

  terminal.show_cursor()?;

  if let Err(err) = res {
    println!("{:?}", err)
  }

  Ok(())
}
