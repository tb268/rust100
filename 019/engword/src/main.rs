use csv::{ReaderBuilder, Trim};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;
use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let csv_path = env::args()
        .nth(1)
        .unwrap_or_else(|| "words.csv".to_string());

    let mut words = load_words(&csv_path)?;
    if words.is_empty() {
        println!("単語が読み込めませんでした。words.csv を確認してください。");
        return Ok(());
    }

    println!("--- 英単語暗記ツール ---");
    println!("対象ファイル: {}（全 {} 単語）", csv_path, words.len());
    println!("Enter を押すと日本語訳が表示されます。");
    println!("覚えたら y、覚えていなければ n を入力してください。\n");

    let mut rng = thread_rng();
    let mut current_round = words.clone();

    loop {
        current_round.shuffle(&mut rng);
        let mut review_list = Vec::new();

        for word in &current_round {
            println!("英単語: {}", word.english);
            prompt_enter("訳を見るには Enter を押してください…")?;
            println!("日本語: {}", word.japanese);

            if !prompt_yes_no("覚えた？ (y/n): ")? {
                review_list.push(word.clone());
            }
            println!();
        }

        if review_list.is_empty() {
            println!("全ての単語を覚えました！お疲れさまでした 🎉");
            break;
        } else {
            println!(
                "{} 個の単語が未習得です。復習ラウンドを開始します。\n",
                review_list.len()
            );
            current_round = review_list;
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct Word {
    english: String,
    japanese: String,
}

fn load_words(path: &str) -> Result<Vec<Word>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .has_headers(true)
        .from_path(path)?;

    let mut words = Vec::new();
    for record in reader.records() {
        let record = record?;
        if record.len() < 2 {
            eprintln!(
                "無効な行をスキップしました: {}",
                record.iter().collect::<Vec<_>>().join(",")
            );
            continue;
        }
        let english = record.get(0).unwrap_or_default().to_string();
        let japanese = record.get(1).unwrap_or_default().to_string();
        if english.is_empty() || japanese.is_empty() {
            eprintln!(
                "空の項目を含む行をスキップしました: {}",
                record.iter().collect::<Vec<_>>().join(",")
            );
            continue;
        }
        words.push(Word { english, japanese });
    }

    Ok(words)
}

fn prompt_enter(message: &str) -> io::Result<()> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(())
}

fn prompt_yes_no(message: &str) -> io::Result<bool> {
    loop {
        print!("{}", message);
        io::stdout().flush()?;

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        match buffer.trim().to_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => {
                println!("y か n で入力してください。");
            }
        }
    }
}
