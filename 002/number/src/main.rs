use rand::Rng;

fn main() {
    let mut rnd= rand::thread_rng();
    let n: i32= rnd.gen_range(1..101);
    println!("0から100までの数値で当ててください");
    loop{
        let mut input =String::new();
        //ここで標準入力
        std::io::stdin().read_line(&mut input).expect("入力失敗");
        let number: i32= match input.trim().parse(){
            Ok(num) => num,
            Err(_)=>{
                println!("数値を入力してください");
                continue;   
            }
       };
       if number<0 ||number>100{
        println!("0から100までの数値にしてください");
        continue;   

       }
        println!("{}",number);
        if n==number{
            println!("正解です！");
            break;
        }else if n>number{
            println!("もっと大きいです");
    
        }
        else {
            println!("もっと小さいです");
    
        }


    }


}
