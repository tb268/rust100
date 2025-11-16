use clap::{Parser, Subcommand};
use std::process::Command;
use std::str;

#[derive(Parser)]
#[command(name = "gitlog")]
#[command(about = "Gitログを解析するCLIツール")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// 表示するコミット数の制限
    #[arg(short, long, default_value = "10")]
    limit: usize,

    /// 開始日時（例: "2024-01-01"）
    #[arg(long)]
    since: Option<String>,

    /// 終了日時（例: "2024-12-31"）
    #[arg(long)]
    until: Option<String>,

    /// 特定のファイルパスでフィルタリング
    #[arg(short, long)]
    file: Option<String>,

    /// 著者でフィルタリング
    #[arg(short, long)]
    author: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// コミット履歴を表示
    Log,
    /// 統計情報を表示
    Stats,
    /// 著者別の統計情報を表示
    AuthorStats,
}

fn main() {
    let cli = Cli::parse();

    // Gitリポジトリかどうか確認
    if !is_git_repo() {
        eprintln!("エラー: これはGitリポジトリではありません");
        std::process::exit(1);
    }

    match cli.command {
        Some(Commands::Log) | None => {
            show_log(&cli);
        }
        Some(Commands::Stats) => {
            show_stats(&cli);
        }
        Some(Commands::AuthorStats) => {
            show_author_stats();
        }
    }
}

fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .is_ok()
}

fn show_log(cli: &Cli) {
    let mut git_args: Vec<String> = vec!["log".to_string(), "--oneline".to_string(), "--decorate".to_string()];

    if let Some(ref since) = cli.since {
        git_args.push("--since".to_string());
        git_args.push(since.clone());
    }

    if let Some(ref until) = cli.until {
        git_args.push("--until".to_string());
        git_args.push(until.clone());
    }

    if let Some(ref author) = cli.author {
        git_args.push("--author".to_string());
        git_args.push(author.clone());
    }

    if let Some(ref file) = cli.file {
        git_args.push("--".to_string());
        git_args.push(file.clone());
    }

    git_args.push("-n".to_string());
    git_args.push(cli.limit.to_string());

    let output = Command::new("git")
        .args(git_args.iter().map(|s| s.as_str()))
        .output()
        .expect("gitコマンドの実行に失敗しました");

    if !output.status.success() {
        eprintln!("エラー: git logの実行に失敗しました");
        std::process::exit(1);
    }

    let log = str::from_utf8(&output.stdout).expect("UTF-8のデコードに失敗しました");
    println!("{}", log);
}

fn show_stats(cli: &Cli) {
    let mut git_args: Vec<String> = vec!["log".to_string(), "--stat".to_string(), "--shortstat".to_string()];

    if let Some(ref since) = cli.since {
        git_args.push("--since".to_string());
        git_args.push(since.clone());
    }

    if let Some(ref until) = cli.until {
        git_args.push("--until".to_string());
        git_args.push(until.clone());
    }

    if let Some(ref author) = cli.author {
        git_args.push("--author".to_string());
        git_args.push(author.clone());
    }

    if let Some(ref file) = cli.file {
        git_args.push("--".to_string());
        git_args.push(file.clone());
    }

    git_args.push("-n".to_string());
    git_args.push(cli.limit.to_string());

    let output = Command::new("git")
        .args(git_args.iter().map(|s| s.as_str()))
        .output()
        .expect("gitコマンドの実行に失敗しました");

    if !output.status.success() {
        eprintln!("エラー: git logの実行に失敗しました");
        std::process::exit(1);
    }

    let stats = str::from_utf8(&output.stdout).expect("UTF-8のデコードに失敗しました");
    println!("{}", stats);
}

fn show_author_stats() {
    let output = Command::new("git")
        .args(["shortlog", "-sn"])
        .output()
        .expect("gitコマンドの実行に失敗しました");

    if !output.status.success() {
        eprintln!("エラー: git shortlogの実行に失敗しました");
        std::process::exit(1);
    }

    let stats = str::from_utf8(&output.stdout).expect("UTF-8のデコードに失敗しました");
    println!("著者別コミット数:");
    println!("{}", stats);
}
