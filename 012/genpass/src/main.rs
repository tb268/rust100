
use clap::Parser;
use rand::*;
use rand::prelude::*;


#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct Args{
    #[arg(short,long, default_value_t = 16)]
    length: u8,
    #[arg(long)]
    no_digits:bool,
    #[arg(long)]
    no_symbols:bool,

}

fn main() {

    println!("Hello, world!");

    let args=Args::parse();
    if args.no_digits {
        println!("数字は除外されます");
    }
    println!("{}",args.length);
    let mut rng = rand::thread_rng();
    let mut chars = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");


    let digit="1234567890";
    let symbols="!@#$%^&*()_+-=[]{}|;:,.<>?" ;
    if !args.no_digits{
        chars.push_str(digit);
    }
    if !args.no_symbols{
        chars.push_str(symbols );
    }
    let char_bytes = chars.as_bytes();
    let pass: String=(0..args.length).map(|_|{
        let randstr=char_bytes.choose(&mut rng).expect(" era-");

        *randstr as char

    }).collect();
    let randomchar=pass;
    println!("パスワードは{}",randomchar);



}
