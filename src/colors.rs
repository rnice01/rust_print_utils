use std::fmt;

#[derive(Copy, Clone)]
pub enum BGColors {
  Black = 40,
  Green = 42,
  Default = 49
}

#[derive(Copy, Clone)]
pub enum FGColors {
  Black = 0,
  Red = 31,
  Green = 32,
  White = 37,
  Default = 39,
  DarkGray = 90
}

#[derive(Copy, Clone)]
pub enum Styles {
  Normal = 0,
  Bold = 1
}

pub fn color_format(text: &str, fg_color: FGColors, bg_color: BGColors, style: Styles) -> String {
  format!("\x1b[{};{};{}m{}\x1b[0m", style as i32, fg_color as i32, bg_color as i32, text)
}

pub struct ColoredText {
  text: String,
  fg_color: FGColors,
  bg_color: BGColors,
  style: Styles
}

impl fmt::Display for ColoredText {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", color_format(&self.text, self.fg_color, self.bg_color, self.style))
  }
}

impl ColoredText {
  pub fn new() -> ColoredText {
    ColoredText{
      text: String::from(""),
      fg_color: FGColors::Default,
      bg_color: BGColors::Default,
      style: Styles::Normal
    }
  }


  pub fn dark_gray(&mut self) -> &mut ColoredText {
    self.fg_color = FGColors::DarkGray;

    self
  }

  pub fn green(&mut self) -> &mut ColoredText {
    self.fg_color = FGColors::Green;

    self
  }

  pub fn on_green(&mut self) -> &mut ColoredText {
    self.bg_color = BGColors::Green;

    self
  }

  pub fn bold(&mut self) -> &mut ColoredText {
    self.style = Styles::Bold;

    self
  }

  pub fn set_text(&mut self, text: &str) -> &mut ColoredText {
    self.text = String::from(text);

    self
  }
}

pub fn colorize(text: &str) -> ColoredText {
  ColoredText{
    text: String::from(text),
    fg_color: FGColors::Default,
    bg_color: BGColors::Default,
    style: Styles::Normal
  }
}