use std::io::Write;
use std::{io, thread};
use std::time::{Duration, Instant};


fn timer(time:i32 ){
    println!("{}秒待ちます",time);
    print!("\r残り{}秒",time);
    io::stdout().flush().expect("msg");
    for i in 1..=time{
        thread::sleep(Duration::from_millis(1000));
        print!("\r残り{}秒",time-i);
        io::stdout().flush().expect("msg");

    }
    println!("\n待機終了");
    return;
}

fn stopwatch(){
    println!("ストップウォッチ開始！(停止の場合には、enterキーを入力してください)");
    let now: Instant= Instant::now();

    let mut i: String=String::new();
    let _ = std::io::stdin().read_line(&mut i);
    
    let era = now.elapsed();
    println!("測定時間は{:?}でした",era);
    return;
}

fn main() {
    println!("モード選択してください{{1:タイマー ,2:ストップウォッチ}}");
    let mut input: String=String::new();
    let mode = match std::io::stdin().read_line(&mut input){
        Ok(_)=> input.to_string().trim().parse::<i32>().unwrap(),
        Err(e)=>{
            println!("数値を入力してください{} ",e);
            panic!("エラー");
        }
    };
    
    if mode==1{
        println!("秒数を教えてください");
        let mut input: String=String::new();
        let time = match std::io::stdin().read_line(&mut input){
            Ok(_)=> input.to_string().trim().parse::<i32>().unwrap(),
            Err(e)=>{
                println!("数値を入力してください{} ",e);
                panic!("エラー");
            }
        };

        timer(time);

    }else if mode==2{
        stopwatch();
    }

    
}
