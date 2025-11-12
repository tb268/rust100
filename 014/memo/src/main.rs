use serde::Serialize;
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::io;


#[derive(Serialize,Deserialize,Debug)]
struct Note{
    content: String,
    timestamp: String,
}

fn main() -> Result<(), io::Error>{
    let note=Note{content: "aaa".to_string(),timestamp:"bcs".to_string()};
    println!("note create{:?}",note);
    let mut file=File::create("notes.json")?;
    file.write_all(serde_json::to_string(&note)?.as_bytes())?;
    Ok(())

    
}
