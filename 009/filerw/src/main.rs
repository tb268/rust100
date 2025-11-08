use std::{fs::File, io::Read};
fn main() {
    let mut f= File::open("texts/test.txt");
    let mut content= String::new();
    f.read_to_string(&mut content).expect("エラー");

    println!( "テキストは{}",content);

} 

