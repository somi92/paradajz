use std::io::{self, stdout, Stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Padding, Paragraph},
    Terminal,
};

use crate::timer::Timer;

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn new() -> Self {
        let terminal = Tui::init().unwrap();

        Tui { terminal }
    }

    pub fn update(&mut self, timer: &Timer) {
        self.terminal
            .draw(|frame| {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Percentage(48),
                        Constraint::Max(1),
                        Constraint::Max(1),
                    ])
                    .split(frame.size());

                let text = format!(
                    "{} - {} {}, {}%",
                    timer.remaining_time_formatted(),
                    char::from_u32(0x23F0).unwrap(),
                    timer.finish_time_formatted(),
                    (timer.percentage_elapsed() * 100.0).floor()
                );

                let text_color = match timer.paused() {
                    true => Color::Rgb(44, 56, 54),
                    false => Color::Rgb(255, 0, 0),
                };
                let progress_color = match timer.paused() {
                    true => Color::Rgb(44, 56, 54),
                    false => Color::Rgb(0, 208, 152),
                };

                frame.render_widget(
                    Paragraph::new(text)
                        .block(
                            Block::default()
                                .borders(Borders::NONE)
                                .padding(Padding::new(1, 1, 0, 0)),
                        )
                        .style(Style::default().fg(text_color)),
                    layout[1],
                );

                frame.render_widget(
                    Gauge::default()
                        .label("")
                        .use_unicode(true)
                        .block(
                            Block::default()
                                .borders(Borders::NONE)
                                .padding(Padding::new(1, 1, 0, 0)),
                        )
                        .gauge_style(Style::default().fg(progress_color).bg(Color::Black))
                        .ratio(timer.percentage_elapsed()),
                    layout[2],
                );
            })
            .unwrap();
    }

    pub fn handle_key_event(&self) -> Option<KeyEvent> {
        if event::poll(std::time::Duration::from_millis(10)).unwrap() {
            if let event::Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    return Some(key);
                }
            }
        }
        None
    }

    pub fn clean_up() -> io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn init() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;

        Ok(terminal)
    }
}
