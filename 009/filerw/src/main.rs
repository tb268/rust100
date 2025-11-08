use std::{fs::File, io::{self, Read}};
fn main()-> Result<(), io::Error> {
    let mut f= File::open("texts/test.txt")?;
    let mut content= String::new();
    f.read_to_string(&mut content)?;

    println!( "テキストは{}",content);
    Ok(())
} 

