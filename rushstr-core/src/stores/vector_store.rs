use crate::SearchOptions;
use crate::stores::store_trait::StoreTrait;

pub struct VectorStore {
    all_items: [&'static str; 103],
}

impl VectorStore {
    pub fn new() -> VectorStore {
        let all_items = [
            "sudo echo \"hola\"",
            "time",
            "ping localhost",
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
        VectorStore { all_items }
    }
}

impl StoreTrait for VectorStore {
    fn filter_items(&self, search_options: &SearchOptions) -> Vec<String> {
        if search_options.text.is_empty() {
            return self.all_items.iter().take(50).map(|item| item.to_string()).collect();
        }

        let input = if search_options.is_case_insensitive() {
            search_options.text.to_lowercase()
        } else {
            search_options.text.to_string()
        };

        self.all_items
            .iter()
            .filter(|item| {
                if search_options.is_case_insensitive() {
                    item.to_lowercase().contains(&input)
                } else {
                    item.contains(&input)
                }
            })
            .map(|item| item.to_string())
            .collect()
    }

    fn total(&self) -> usize {
        self.all_items.len()
    }
}
