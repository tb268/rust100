use clap::Parser;
use std::io;

#[derive(Parser,Debug)]
#[command(version,about,long_about=None)]
struct Args{
    #[arg(short,long)]
    file: String,
}



fn main() ->Result<(), io::Error>{
    let args: Args=Args::parse();
    let test:String=std::fs::read_to_string( args.file)?;
    let num=test.split_whitespace().count();
    println!("num is {}",num);
    Ok(())
}
