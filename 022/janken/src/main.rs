use rand::Rng;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hand {
    Rock,
    Scissors,
    Paper,
}

impl Hand {
    fn from_input(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "g" | "rock" | "ぐー" | "グー" => Some(Self::Rock),
            "c" | "scissors" | "ちょき" | "チョキ" => Some(Self::Scissors),
            "p" | "paper" | "ぱー" | "パー" => Some(Self::Paper),
            _ => None,
        }
    }

    fn random() -> Self {
        match rand::thread_rng().gen_range(0..3) {
            0 => Self::Rock,
            1 => Self::Scissors,
            _ => Self::Paper,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Rock => "グー",
            Self::Scissors => "チョキ",
            Self::Paper => "パー",
        }
    }

    fn battle(&self, opponent: &Self) -> Outcome {
        use Outcome::*;
        match (self, opponent) {
            (a, b) if a == b => Draw,
            (Self::Rock, Self::Scissors)
            | (Self::Scissors, Self::Paper)
            | (Self::Paper, Self::Rock) => Win,
            _ => Lose,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn message(&self) -> &'static str {
        match self {
            Outcome::Win => "あなたの勝ち！",
            Outcome::Lose => "CPUの勝ち…",
            Outcome::Draw => "引き分け！",
        }
    }
}

fn main() {
    println!("=== じゃんけんCLI ===");
    println!("g: グー, c: チョキ, p: パー, exit: 終了\n");

    loop {
        print!("あなたの手を入力してください > ");
        io::stdout().flush().expect("stdout flush failed");

        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_err() {
            eprintln!("入力の読み取りに失敗しました。もう一度試してください。");
            continue;
        }

        let trimmed = buffer.trim();
        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            println!("ゲームを終了します。お疲れさまでした！");
            break;
        }

        let player = match Hand::from_input(trimmed) {
            Some(hand) => hand,
            None => {
                println!("'{trimmed}' は無効な入力です。g/c/p または exit を入力してください。\n");
                continue;
            }
        };

        let cpu = Hand::random();
        println!("あなた: {} vs CPU: {}", player.as_str(), cpu.as_str());

        let outcome = player.battle(&cpu);
        println!("結果: {}", outcome.message());
        println!();
    }
}
