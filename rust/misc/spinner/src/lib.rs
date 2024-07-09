use anyhow::Result;
use std::{
  io::{stdout, Write}, sync::mpsc, thread::{self, JoinHandle}, time::Duration
};

const ESC: char = 27 as char;

pub struct Spinner {
  delay: Duration,
  characters: String, // List of character to use, in the form of a string to be
                      // iterated over
  // Stored transmitter that points to the channel
  transmitter: Option<mpsc::Sender<i32>>,
}

impl Spinner {
  pub fn new(delay: Duration) -> Self {
    Self {
      delay,
      characters: String::from("-\\|/"),
      transmitter: None,
    }
  }

  pub fn start (&mut self) -> JoinHandle<Result<()>> {
    let (tx, rx) = mpsc::channel();
    self.transmitter = Some(tx);

    let delay = self.delay;
    let available_characters = self.characters.clone();

    // Make cursor invisible
    print!("{ESC}[?25l");

    thread::spawn(move || -> Result<()> {
      while rx.try_recv().is_err() {
        for char in available_characters.chars() {
          print!("{}", char);
          print!("{ESC}[1D");
          stdout().flush()?;
          thread::sleep(delay);
        };
      }

      Ok(())
    })
  }

  pub fn stop(&mut self) -> Result<()> {
    if let Some(tx) = &self.transmitter {
      tx.send(0)?;
    }

    // Make cursor visible
    print!("{ESC}[?25h");
    println!();

    Ok(())
  }
}
