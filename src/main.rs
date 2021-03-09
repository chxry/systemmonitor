use std::{
  env,
  io::{stdout, Write},
  thread, time,
};
use sysinfo::{ProcessorExt, SystemExt};
use termion::{async_stdin, clear, cursor, event, input::TermRead, raw::IntoRawMode, screen};

mod utils;

fn main() {
  let mut refresh = 500;
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 {
    if let Ok(v) = args[1].parse::<u64>() {
      refresh = v
    }
  }

  let mut system = sysinfo::System::new_all();
  let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode().unwrap());
  let mut stdin = async_stdin().keys();

  let long_os_version = system.get_long_os_version().unwrap();

  write!(screen, "{}", cursor::Hide).unwrap();

  loop {
    let input = stdin.next();

    if let Some(Ok(event::Key::Char('q'))) = input {
      write!(screen, "{}", cursor::Show).unwrap();
      break;
    }

    system.refresh_all();
    write!(screen, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
    utils::write_top(&mut screen);

    utils::write_header(&mut screen, "INFORMATION");
    utils::write_line(&mut screen, "OS VERSION", &long_os_version);
    utils::write_line(&mut screen, "UPTIME", &system.get_uptime().to_string());

    utils::write_header(&mut screen, "CPU");
    utils::write_line(
      &mut screen,
      "BRAND",
      system.get_global_processor_info().get_brand(),
    );
    utils::write_line(
      &mut screen,
      "USAGE",
      &format!("{:.2}%", system.get_global_processor_info().get_cpu_usage()),
    );
    utils::write_line(&mut screen, "CORES", "");
    for processor in system.get_processors() {
      utils::write_line(
        &mut screen,
        processor.get_name(),
        &format!("{:.2}%", processor.get_cpu_usage()),
      );
    }

    utils::write_header(&mut screen, "MEMORY");
    utils::write_line(
      &mut screen,
      "USED",
      &format!(
        "{}/{}MB",
        system.get_used_memory() / 1024,
        system.get_total_memory() / 1024
      ),
    );

    screen.flush().unwrap();
    thread::sleep(time::Duration::from_millis(refresh));
  }
}
