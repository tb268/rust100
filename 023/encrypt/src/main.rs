mod cipher;

use cipher::{caesar_cipher, Operation};
use std::env;

#[derive(Debug)]
struct Config {
    operation: Operation,
    text: String,
    shift: u8,
}

impl Config {
    fn parse_args(args: Vec<String>) -> Result<Self, String> {
        if args.len() < 4 {
            return Err(format!(
                "使用方法: {} <encrypt|decrypt> <テキスト> <シフト数>\n\
                例: {} encrypt \"Hello World\" 3",
                args[0], args[0]
            ));
        }

        let operation = match args[1].to_lowercase().as_str() {
            "encrypt" | "e" => Operation::Encrypt,
            "decrypt" | "d" => Operation::Decrypt,
            _ => return Err(format!("無効な操作: {}. encrypt または decrypt を指定してください。", args[1])),
        };

        let text = args[2].clone();

        let shift = args[3]
            .parse::<u8>()
            .map_err(|_| format!("シフト数は0-25の範囲の数値である必要があります: {}", args[3]))?;

        if shift > 25 {
            return Err(format!("シフト数は0-25の範囲である必要があります: {}", shift));
        }

        Ok(Config {
            operation,
            text,
            shift,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = match Config::parse_args(args) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("エラー: {}", e);
            std::process::exit(1);
        }
    };

    let result = caesar_cipher(&config.text, config.shift, config.operation);
    
    println!("元のテキスト: {}", config.text);
    println!("操作: {:?}", config.operation);
    println!("シフト数: {}", config.shift);
    println!("結果: {}", result);
}
