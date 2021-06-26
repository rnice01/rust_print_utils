use std::time::Duration;
use std::thread;
use crate::colors::{ColoredText};

pub struct Typewriter {
  pause_for: Duration
}

impl Typewriter {
  pub fn new() -> Typewriter {
    Typewriter{
      pause_for: Duration::from_millis(100)
    }
  }

  pub fn set_pause(&mut self, duration : Duration) -> &Self {
    self.pause_for = duration;

    self
  }

  pub fn print(&self, colored_text : &ColoredText) {
    println!("{}", colored_text);
    thread::sleep(self.pause_for);
  }
}