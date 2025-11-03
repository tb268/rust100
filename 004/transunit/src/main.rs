enum Unit{
    M,
    Km,
    C,
    F,
    G,
    Kg,
}

fn unit_set(unit: &Unit, number: &f32 ){
    match unit{
        Unit::M=> println!("{} km",((*number)/1000.0)),
        Unit::Km=>println!("{} m",((*number)*1000.0)),
        Unit::C=>println!("{} F",((*number)*9.0/5.0+32.0)),
        Unit::F=>println!("{} C",(((*number)-32.0)*5.0/9.0)),
        Unit::G=>println!("{} kg",((*number)/1000.0)),
        Unit::Kg=>println!("{} g",((*number)*1000.0)),
    }
}

#[derive(Debug)]
struct Item<'a>{
    size_input: Option<&'a str>,
    unit_input: Option<&'a str>,
}

impl Item<'_>{
    fn transform(&self)->i32{
        let num=match self.size_input{
            Some(v) => match  v.parse::<f32>(){
                Ok(n) => n,
                Err(_) => {
                    println!("失敗");
                    return 0;
                }
            },
            None=>{
                println!("指定なし");
                return 0;
            }
        };

        let unit = match self.unit_input{
            Some("m")=>Unit::M,
            Some("km")=>Unit::Km,
            Some("C")=>Unit::C,
            Some("F")=>Unit::F,
            Some("g")=>Unit::G,
            Some("kg")=>Unit::Kg,
            _=>{
                println!("エラ〜");
                return 0;
            }
        };
        unit_set(&unit,&num);
        1
    }
}

fn main() {
    println!("変換したいものを入力してください");
    let mut input=String::new();
    std::io::stdin().read_line(&mut input).expect("入力失敗");

    let mut s=input.split_whitespace();
 
    let aaa=Item{size_input: s.next(),unit_input: s.next()};
    aaa.transform();
    println!("{:?}",aaa);
}
