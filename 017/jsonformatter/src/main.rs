use clap::Parser;
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "jsonformatter")]
#[command(about = "JSONデータをインデント付き形式で出力するCLIツール")]
struct Args {
    /// JSONファイルのパス（指定しない場合は標準入力から読み込み）
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    
    let json_input = if let Some(file_path) = args.file {
        // ファイルパスが指定されている場合
        fs::read_to_string(&file_path).expect("ファイルを読み込めませんでした")
    } else {
        // 標準入力から読み込む
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .expect("標準入力から読み込めませんでした");
        buffer
    };

    // JSONをパースして整形
    let json_value: serde_json::Value = serde_json::from_str(&json_input)
        .expect("無効なJSON形式です");

    // インデント付きで出力
    let formatted = serde_json::to_string_pretty(&json_value)
        .expect("JSONのフォーマットに失敗しました");

    println!("{}", formatted);
}
