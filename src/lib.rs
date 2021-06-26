pub mod colors;
pub mod typewriter;

#[cfg(test)]
mod tests {
  use super::colors::{color_format, colorize, FGColors, BGColors, Styles};

  #[test]
  fn test_color_formatter() {
      let style = Styles::Bold;
      let fg_color = FGColors::DarkGray;
      let bg_color = BGColors::Green;
      let expected = format!("\x1b[{};{};{}msome text\x1b[0m", style as i32, fg_color as i32, bg_color as i32);

      assert_eq!(color_format("some text", fg_color, bg_color, style), expected);
  }

  #[test]
  fn test_only_fg_color_set() {
      let green_text = colorize("some text").green().to_string();
      let expected = color_format("some text", FGColors::Green, BGColors::Default, Styles::Normal);

      assert_eq!(green_text, expected);
  }

  #[test]
  fn test_only_style_set() {
      let bold_text = colorize("some text").bold().to_string();
      let expected = color_format("some text", FGColors::Default, BGColors::Default, Styles::Bold);

      assert_eq!(bold_text, expected);
  }

  #[test]
  fn test_only_bg_set() {
      let default_on_green = colorize("some text").on_green().to_string();
      let expected = color_format("some text", FGColors::Default, BGColors::Green, Styles::Normal);

      assert_eq!(default_on_green, expected);
  }

  #[test]
  fn test_combinations() {
      let green_bold_on_default = colorize("some text").green().bold().to_string();
      let expected = color_format("some text", FGColors::Green, BGColors::Default, Styles::Bold);

      assert_eq!(green_bold_on_default, expected);
  }
}
