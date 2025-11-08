fn run_test(testname: &str ,func :fn(x: i32)-> bool){
    if func(1){
        println!("testname {}:OK",testname);
    }else{
        println!("testname {}:NG",testname);
    }
}

fn main() {
    println!("Hello, world!");
    run_test("mytest", |x| x+1==3);
}
