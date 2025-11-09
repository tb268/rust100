


struct Config{
    name: String,
    n: i32,
    verbose: bool
}


fn main() {
    let mut args=std::env::args().skip(1);
    let mut name : Option<String>=None;

    let mut n: Option<i32>=None;
    let mut verbose:bool=false;

    while let Some(arg) = args.next(){
        match arg.as_str(){
            "--name" => {
                if let Some(value) = args.next() {
                    name=Some(value);
                }else{
                    panic!("エラ〜")
                }
            },
            "-n"=>{
                if let Some(num) = args.next(){
                    match num.parse::<i32>(){
    
                        Ok(num)=> {
                            n = Some(num);
                        },
                        Err(_)=>{
                            println!("nの引数は数値にしてください");
                        }
                    }
                }

            },
            "--verbose"=>{
                verbose=true;
            },
            _=>{
                println!("無効なオプションが含まれています。")
            }
        }

    }
    
    let outname =match name{
        Some(name)=>name,
        None=>panic!("エラー")
    };
    let outn =match n{
        Some(n)=>n,
        None=>panic!("エラー")
    };
    let config1=Config{name: outname,n: outn,verbose: verbose};

    println!("{:?},{:?},{:?}",config1.name,config1.n,config1.verbose);
}
