use std::io::Write;
use termion::{color, style, terminal_size};

pub fn write_top(screen: &mut Write) {
  let exit_message = "Press Q to exit ";
  write!(
    screen,
    "{}{}{}{}{}\r\n",
    color::Bg(color::LightGreen),
    color::Fg(color::Black),
    format!(
      "{:width$}",
      "SYSTEM MONITOR:",
      width = usize::from(terminal_size().unwrap().0) - exit_message.chars().count(),
    ) + exit_message,
    color::Bg(color::Reset),
    color::Fg(color::Reset),
  )
  .unwrap();
}

pub fn write_header(screen: &mut Write, header: &str) {
  write!(
    screen,
    "\n{}{}: {}\r\n",
    color::Fg(color::LightGreen),
    header,
    color::Fg(color::Reset),
  )
  .unwrap()
}

pub fn write_line(screen: &mut Write, label: &str, content: &str) {
  write!(
    screen,
    "{}{}:{} {}\r\n",
    style::Bold,
    label,
    style::Reset,
    content
  )
  .unwrap()
}
