use crate::{HItem, HScanner};

pub struct MemCrawler {
    all_items: Vec<HItem>,
}

impl MemCrawler {
    pub fn new() -> Self {
        let all_items = vec![
            "sudo echo \"hola\"".to_string(),
            "time".to_string(),
            "ping localhost".to_string(),
            "export ORACLE_HOME=/uuid/dev/sg/dbhome_1\nexport PATH=ORACLE_HOME/bin:$PATH\nexport ORACLE_SID=abc"
                .to_string(),
            "git status".to_string(),
            "git commit -m 'Initial commit'".to_string(),
            "git push origin main".to_string(),
            "git checkout -b feature/new-ui".to_string(),
            "git rebase -i HEAD~3".to_string(),
            "git merge develop".to_string(),
            "git log --oneline".to_string(),
            "ls -la".to_string(),
            "ls -lh".to_string(),
            "cd ~/projects/rushstr".to_string(),
            "cd ..".to_string(),
            "pwd".to_string(),
            "source ~/venv/bin/activate\nexport PYTHONPATH=~/projects/myapp\nexport DEBUG=true".to_string(),
            "export KUBECONFIG=~/.kube/config-dev\nexport CONTEXT=my-cluster\nkubectl config use-context $CONTEXT".to_string(),
            "\
            abc text\
            this is a large text\
            abc\
            \
            ".to_string(),
            "cargo build".to_string(),
            "cargo run".to_string(),
            "cargo test --release".to_string(),
            "cargo fmt --check".to_string(),
            "cargo clippy".to_string(),
            "docker run -p 8080:80 -v $(pwd):/app \nnginx:latest".to_string(),
            "find . -type f -name \"*.rs\" | \nxargs grep \"fn main\"".to_string(),
            "git log --pretty=format:\"%h - %an, %ar : %s\" \n          --graph \n      --since=2.weeks".to_string(),
            "cat /var/log/syslog | \ngrep ERROR | \ntail -n 100".to_string(),
            "ssh user@hostname \n\"cd /var/www && \n   ls -la && \n    tail -f error.log\"".to_string(),
            "awk 'BEGIN {sum=0} \n      {sum+=$1} \nEND {print sum}' data.txt".to_string(),
            "cargo doc --open".to_string(),
            "docker ps".to_string(),
            "docker-compose up -d".to_string(),
            "docker logs -f app_container".to_string(),
            "docker exec -it container_id bash".to_string(),
            "htop".to_string(),
            "top".to_string(),
            "kubectl get pods".to_string(),
            // Example 3: PostgreSQL database environment
            "export PGDATA=/var/lib/postgresql/13/main\nexport PATH=/usr/lib/postgresql/13/bin:$PATH\nexport PGDATABASE=mydb".to_string(),
            "kubectl describe pod my-app-pod".to_string(),
            "kubectl logs -f my-app-pod".to_string(),
            "ssh user@host".to_string(),
            "scp ./target/release/rushstr user@host:/usr/local/bin/".to_string(),
            "make install".to_string(),
            "make build".to_string(),
            "make clean".to_string(),
            "nvim src/main.rs".to_string(),
            "vim Cargo.toml".to_string(),
            "nano README.md".to_string(),
            "grep -rn 'TODO' ./src".to_string(),
            "tail -f /var/log/syslog".to_string(),
            "systemctl restart nginx".to_string(),
            "curl http://localhost:8080/health".to_string(),
            "ping 8.8.8.8".to_string(),
            "chmod +x deploy.sh".to_string(),
            "rsync -avz ./data user@host:/backups/".to_string(),
            "python3 script.py".to_string(),
            "node server.js".to_string(),
            "npm install".to_string(),
            // Example 2: Java environment setup
            "export JAVA_HOME=/usr/lib/jvm/java-17-openjdk-amd64\nexport PATH=$JAVA_HOME/bin:$PATH\nexport APP_ENV=production".to_string(),
            "yarn start".to_string(),
            "composer install".to_string(),
            "bundle exec jekyll serve".to_string(),
            "mvn clean install".to_string(),
            "gradle build".to_string(),
            "cmake ..".to_string(),
            "make -j4".to_string(),
            "sudo apt update".to_string(),
            "sudo apt upgrade".to_string(),
            "brew update".to_string(),
            "brew upgrade".to_string(),
            "pip install -r requirements.txt".to_string(),
            "pipenv install".to_string(),
            "poetry install".to_string(),
            "java -version".to_string(),
            "SELECT * FROM users\nWHERE age > 18\nORDER BY last_name".to_string(),
            "rustup update".to_string(),
            "rustup component add clippy".to_string(),
            "rustup component add rustfmt".to_string(),
            "rbenv install 2.7.0".to_string(),
            "rbenv global 2.7.0".to_string(),
            "go run main.go".to_string(),
            "while true; do\n  echo \"still running...\"\n  sleep 1\ndone".to_string(),
            "go build -o app".to_string(),
            "go test ./...".to_string(),
            "curl -X POST \n-H \"Content-Type: application/json\" \n-d '{\"key\": \"value\"}' \nhttps://api.example.com/endpoint".to_string(),
            "php artisan serve".to_string(),
            "composer dump-autoload".to_string(),
            "lsof -i :3000".to_string(),
            "ffmpeg -i input.mp4 \n-c:v libx264 -crf 23 \n-c:a aac -b:a 128k \noutput.mp4".to_string(),
            "netstat -tulnp".to_string(),
            "ifconfig".to_string(),
            "ip a".to_string(),
            "systemctl status docker".to_string(),
            "journalctl -u nginx".to_string(),
            "sudo reboot".to_string(),
            "sudo shutdown -h now".to_string(),
            "screen -ls".to_string(),
            "python -c \"import sys; \nprint(sys.version); \nprint('Hello, world!')\"".to_string(),
            "tmux new -s session".to_string(),
            "tmux attach -t session".to_string(),
            "ps aux | grep rust".to_string(),
            "kill -9 12345".to_string(),
            "alias ll='ls -la'".to_string(),
            "history | grep cargo".to_string(),
            "date".to_string(),
            "cal".to_string(),
            "df -h".to_string(),
            "du -sh *".to_string(),
            "free -m".to_string(),
            "top -o %MEM".to_string(),
            "curl ifconfig.me".to_string(),
            "wget https://example.com/file.zip".to_string(),
            "unzip file.zip".to_string(),
            "tar -xzvf archive.tar.gz".to_string(),
            "scp file.zip user@host:/tmp/".to_string(),
            "chown -R user:user /var/www".to_string(),
            "ln -s /path/to/target symlink".to_string(),
            "find . -name '*.rs'".to_string(),
            "for i in {1..10}; do\n  echo \"Processing item $i\"\n  sleep 0.5\ndone".to_string(),
            "cargo install cargo-watch".to_string(),
            "export NODE_ENV=development
export DATABASE_URL=\"postgresql://user:pass@localhost:5432/mydb\"
export LOG_LEVEL=debug".to_string(),
        ];
        let mut h_items = Vec::new();
        for item in all_items {
            let cmds = item.split("\n").map(|m| m.to_string()).collect::<Vec<_>>();
            h_items.push(HItem::new(cmds))
        }
        Self { all_items: h_items }
    }
}

impl HScanner for MemCrawler {
    fn load(&self) -> Vec<HItem> {
        self.all_items.clone()
    }
}
