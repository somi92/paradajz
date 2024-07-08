use chrono::Duration;
use clap::Parser;
use notificator::Notificator;
use ratatui::crossterm::event::KeyCode;
use timer::Timer;
use tui::Tui;

mod notificator;
mod timer;
mod tui;

/// Simple timer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Duration of interval in minutes
    #[arg(short, long, default_value = "20")]
    duration: i64,
}

fn main() {
    let args = Args::parse();

    let duration_ms = Duration::minutes(args.duration).num_milliseconds();
    let mut timer = Timer::new(duration_ms);

    let mut tui = Tui::new();

    loop {
        timer.tick();

        if timer.is_expired() || timer.terminated() {
            break;
        }

        tui.update(&timer);

        match tui.handle_key_event() {
            Some(key_event) => match key_event.code {
                KeyCode::Char('p') => timer.toggle_pause(),
                KeyCode::Char('q') => timer.terminate(),
                KeyCode::Char('r') => timer.reset(duration_ms),
                _ => (),
            },
            None => (),
        };
    }

    if !timer.terminated() {
        Notificator::show_notification("Interval expired.", 0);
    }
    Tui::clean_up().unwrap();
}
