

fn main() {
    println!("整数を入力してください");
    let mut input=String::new();
    let num: i32;
    loop{
        input.clear();
        std::io::stdin().read_line(&mut input).expect("入力失敗");
        match input.trim().parse(){
            Ok(n)=>{
                num=n;
                break
            },
            Err(_)=>{
                println!("数値を入力してください");
                continue
            }
        };
    }

    for i in 1..=num{

        if i%15==0{
            println!("FizzBuzz");
        }else if i%3==0{
            println!("Fizz");

        }else if i%5==0{
            println!("Buzz");

        }else{
            println!("{}",i);
        }
    }
}
