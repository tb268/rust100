use rand::Rng;

fn main() {
    let args=std::env::args().collect::<Vec<String>>();

    let min_num :i32=match args[1].parse::<i32>(){
        Ok(n)=>n,
        Err(e)=>{
            println!("エラ〜です");
            panic!("エラー");
        }
    };
    let max_num :i32=match args[2].parse::<i32>(){
        Ok(n)=>n,
        Err(e)=>{
            println!("エラ〜です");
            panic!("エラー");
        }
    };

    println!("生成された乱数：min{:?},max{:?}",min_num,max_num);
    let mut rng=rand::thread_rng();
    let ans: i32= rng.gen_range(min_num..=max_num);
    println!("生成された乱数：{}",ans);
}
