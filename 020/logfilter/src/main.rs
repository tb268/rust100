use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "ログファイルをキーワードでフィルタするCLI", long_about = None)]
struct Args {
    /// 読み込むログファイルのパス
    #[arg(short, long)]
    file: PathBuf,

    /// 検索対象のキーワード
    #[arg(short, long)]
    keyword: String,

    /// キーワードに一致しない行のみを表示する
    #[arg(short = 'v', long = "invert")]
    invert: bool,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("エラー: {err}");
        process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(&args.file)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let contains = line.contains(&args.keyword);
        let should_print = if args.invert { !contains } else { contains };

        if should_print {
            println!("{line}");
        }
    }

    Ok(())
}
