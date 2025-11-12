use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::io;
use clap::Parser;
use clap::command;
use chrono::Local; // <-- chrono を追加

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct Args{
    #[arg(short,long)]
    add: Option<String>, // 値がない場合を考慮し Option<String> に変更
    #[arg(short,long)]
    list: bool,
}

#[derive(Serialize,Deserialize,Debug)]
struct Note{
    content: String,
    timestamp: String,
}

fn main() -> Result<(), io::Error>{
    // 冗長な初期化を削除
    let args=Args::parse();
    let file_path = "notes.json";

    // 1. ファイルの読み込みとデシリアライズ
    let json_data = std::fs::read_to_string(file_path);

    let mut notes: Vec<Note> = match json_data {
        Ok(data) => {
            // 💡 修正 2: unwrap() を使う場合は Result<T, E> 型が一致するように調整
            // ここでは serde_json::Error を io::Error に変換できないため、
            // unwrap_or_else でエラーハンドリングし、Vec<Note> を返すようにする
            serde_json::from_str(&data).unwrap_or_else(|e| {
                eprintln!("Error deserializing notes: {}", e);
                Vec::new() // デシリアライズに失敗したら空のリストを返す
            })
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // ファイルが存在しない場合は空のリストを返す (正常な処理)
            Vec::new()
        }
        Err(e) => {
            // その他のIOエラーの場合はエラーを返す
            return Err(e);
        }
    };

    // 2. メモの追加 (addが指定された場合のみ)
    if let Some(content) = args.add { // 💡 修正 3: Option<String> に合わせたチェック
        // 💡 修正 4: リアルタイムのタイムスタンプを使用
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        notes.push(Note{content, timestamp: now});

        // 追加した場合は、必ずファイルを保存する必要があるため、以下の保存ロジックに任せる
    }
    
    // 3. リスト表示 (listが指定された場合のみ)
    if args.list { // 💡 修正 5: 読み込んだ notes を表示
        println!("\n--- Rust Memo List ---");
        for note in &notes {
            println!("[{}] {}", note.timestamp, note.content);
        }
        println!("----------------------\n");
    }

    // 4. ファイルへの書き込み（追加・変更があった場合に実行される）
    let path = std::path::Path::new(file_path);

    // ディレクトリ作成と書き込みのロジックは流用
    match path.parent(){
        Some(parent)=>{
            std::fs::create_dir_all(parent)?;
            let mut file=File::create(path)?;
            file.write_all(serde_json::to_string_pretty(&notes)?.as_bytes())?; // 整形して書き込み
        },
        None=>{
            // notes.json がカレントディレクトリにある場合は親ディレクトリがないため、
            // ファイルの存在確認と作成は省略できることが多いですが、ロジックを維持します。
            let mut file=File::create(path)?;
            file.write_all(serde_json::to_string_pretty(&notes)?.as_bytes())?;
        }
    }
    
    // 初期の println!("note create{:?}",notes); は不要なので削除またはコメントアウト
    
    Ok(())
}