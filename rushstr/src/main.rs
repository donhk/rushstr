use anyhow::Result;
use ratatui::crossterm::event::KeyModifiers;
use ratatui::layout::{Constraint, Direction, Layout, Position};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode, KeyEventKind},
    widgets::Paragraph,
};

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut text = String::new();
    let all_items = [
        "git status",
        "git commit -m 'Initial commit'",
        "git push origin main",
        "git checkout -b feature/new-ui",
        "git rebase -i HEAD~3",
        "git merge develop",
        "git log --oneline",
        "ls -la",
        "ls -lh",
        "cd ~/projects/rushstr",
        "cd ..",
        "pwd",
        "cargo build",
        "cargo run",
        "cargo test --release",
        "cargo fmt --check",
        "cargo clippy",
        "cargo doc --open",
        "docker ps",
        "docker-compose up -d",
        "docker logs -f app_container",
        "docker exec -it container_id bash",
        "htop",
        "top",
        "kubectl get pods",
        "kubectl describe pod my-app-pod",
        "kubectl logs -f my-app-pod",
        "ssh user@host",
        "scp ./target/release/rushstr user@host:/usr/local/bin/",
        "make install",
        "make build",
        "make clean",
        "nvim src/main.rs",
        "vim Cargo.toml",
        "nano README.md",
        "grep -rn 'TODO' ./src",
        "tail -f /var/log/syslog",
        "systemctl restart nginx",
        "curl http://localhost:8080/health",
        "ping 8.8.8.8",
        "chmod +x deploy.sh",
        "rsync -avz ./data user@host:/backups/",
        "python3 script.py",
        "node server.js",
        "npm install",
        "yarn start",
        "composer install",
        "bundle exec jekyll serve",
        "mvn clean install",
        "gradle build",
        "cmake ..",
        "make -j4",
        "sudo apt update",
        "sudo apt upgrade",
        "brew update",
        "brew upgrade",
        "pip install -r requirements.txt",
        "pipenv install",
        "poetry install",
        "java -version",
        "rustup update",
        "rustup component add clippy",
        "rustup component add rustfmt",
        "rbenv install 2.7.0",
        "rbenv global 2.7.0",
        "go run main.go",
        "go build -o app",
        "go test ./...",
        "php artisan serve",
        "composer dump-autoload",
        "lsof -i :3000",
        "netstat -tulnp",
        "ifconfig",
        "ip a",
        "systemctl status docker",
        "journalctl -u nginx",
        "sudo reboot",
        "sudo shutdown -h now",
        "screen -ls",
        "tmux new -s session",
        "tmux attach -t session",
        "ps aux | grep rust",
        "kill -9 12345",
        "alias ll='ls -la'",
        "history | grep cargo",
        "date",
        "cal",
        "df -h",
        "du -sh *",
        "free -m",
        "top -o %MEM",
        "curl ifconfig.me",
        "wget https://example.com/file.zip",
        "unzip file.zip",
        "tar -xzvf archive.tar.gz",
        "scp file.zip user@host:/tmp/",
        "chown -R user:user /var/www",
        "ln -s /path/to/target symlink",
        "find . -name '*.rs'",
        "cargo install cargo-watch",
    ];

    let mut selected = 0;
    loop {
        let filtered: Vec<_> = all_items.iter().filter(|item| item.contains(&text)).cloned().collect();
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                .split(frame.area());

            // Render the search bar
            let search =
                Paragraph::new(Line::from(text.as_str())).block(Block::default().borders(Borders::ALL).title("Search"));
            frame.render_widget(search, layout[0]);

            // Show the cursor inside the search bar
            let x_offset = text.chars().count(); // Use byte offset if using Unicode-aware rendering
            let cursor_x = layout[0].x + x_offset as u16 + 1; // +1 to account for left border
            let cursor_y = layout[0].y + 1; // +1 to be inside the border
            frame.set_cursor_position(Position::new(cursor_x, cursor_y));

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
                    ListItem::new(*item).style(style)
                })
                .collect();

            let list = List::new(list_items).block(Block::default().borders(Borders::TOP).title("Results"));
            frame.render_widget(list, layout[1]);
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Esc {
                    return Ok(());
                }
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    return Ok(());
                }
                if key.code == KeyCode::Up && selected > 0 {
                    selected -= 1;
                }
                if key.code == KeyCode::Down && selected + 1 < all_items.iter().filter(|s| s.contains(&text)).count() {
                    selected += 1;
                }
                if key.code == KeyCode::Enter {
                    let filtered: Vec<_> = all_items.iter().filter(|item| item.contains(&text)).cloned().collect();
                    if let Some(cmd) = filtered.get(selected) {
                        println!("\nSelected command: {}", cmd);
                        return Ok(());
                    }
                }
                if let KeyCode::Char(c) = key.code {
                    text.push(c);
                    selected = 0;
                }
                if key.code == KeyCode::Backspace {
                    text.pop();
                    selected = 0;
                }
            }
        }
    }
}
