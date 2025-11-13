use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Data{
    name: String,
    age: i32,
    email: String,
}

fn main()->Result<(), Box<dyn std::error::Error>> {

    let file_path = "data.csv"; 
    let mut reader = Reader::from_path(file_path)?;
    for result in reader.deserialize::<Data>(){
        let data: Data = result?;
        println!("{:?}",data);
    }
    Ok(())
}
