use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::crossterm::event::KeyEventKind;
use ratatui::text::Line;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io;
use std::time::Duration;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    result
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut input = String::new();
    let all_items = [
        "git status",
        "git commit -m 'Initial commit'",
        "ls -la",
        "cargo build",
        "cargo test",
        "docker ps",
        "htop",
        "kubectl get pods",
        "ssh user@host",
        "make install",
    ];
    let mut selected = 0;

    loop {
        let filtered: Vec<_> = all_items.iter().filter(|item| item.contains(&input)).cloned().collect();

        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Length(2), Constraint::Min(1)].as_ref())
                .split(frame.size());

            let search = Paragraph::new(Line::from(input.as_str()));
            frame.render_widget(search, layout[0]);

            let list_items: Vec<ListItem> = filtered
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let style = if i == selected {
                        Style::default()
                            .bg(Color::Gray)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(item.clone()).style(style)
                })
                .collect();

            let list = List::new(list_items).block(Block::default().borders(Borders::TOP).title("Results"));
            frame.render_widget(list, layout[1]);
        })?;

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                        selected = 0;
                    },
                    KeyCode::Backspace => {
                        input.pop();
                        selected = 0;
                    },
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    },
                    KeyCode::Down => {
                        if selected + 1 < all_items.iter().filter(|s| s.contains(&input)).count() {
                            selected += 1;
                        }
                    },
                    KeyCode::Enter => {
                        let filtered: Vec<_> = all_items.iter().filter(|item| item.contains(&input)).cloned().collect();
                        if let Some(cmd) = filtered.get(selected) {
                            println!("\nSelected command: {}", cmd);
                        }
                        break;
                    },
                    KeyCode::Esc => break,
                    _ => {},
                },
                _ => {},
            }
        }
    }

    Ok(())
}
