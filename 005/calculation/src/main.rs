#[derive(Debug)]
enum CalcType{
    Plus,
    Minus,
    Mul,
    Div,
}


fn calc_func(calcStr: &str)-> Result<CalcType, String>{
    match calcStr{
        "+" =>Ok(CalcType::Plus),
        "-"=>Ok(CalcType::Minus),
        "*"=>Ok(CalcType::Mul),
        "/"=>Ok(CalcType::Div),
        _=> Err(format!("計算が不正です")),
    }
}

fn calc_ans(calc_type: &CalcType,num1: i32,num2: i32)->i32{
    let ans=match calc_type{
        CalcType::Plus=>num1 + num2,
        CalcType::Minus=>num1 - num2,
        CalcType::Mul=>num1 * num2,
        CalcType::Div=>num1 / num2,
    };
    return ans;
    
}


fn main() {
    println!("計算します");
    let args=std::env::args().collect::<Vec<String>>();

    let num_l: i32 = match args[1].to_string().parse::<i32>(){
        Ok(num)=>{
            num
        },
        Err(e)=>{
            println!("エラ〜");
            return
        }
    };
    let calc: &str = &args[2];
    let num_r: i32 = match args[3].to_string().parse::<i32>(){
        Ok(num)=>{
            num
        },
        Err(e)=>{
            println!("エラ〜");
            return
        }
    };



    let a: CalcType;
    a= match calc_func(calc){
        Ok(calc_type)=>{
            calc_type
        },
        Err(e)=>{
            println!("エラ〜");
            return
        }
    }; 
    
    let answer=calc_ans(&a,num_l,num_r);
    println!("{}",answer);

}
