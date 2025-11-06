
#[derive(Debug)]
struct Todo{
    title: String,
    done: bool,
}


fn main() {
    let mut todos: Vec<Todo>=Vec::new();
    loop{
        println!("タスクを入力してください");
        
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        


        if let Some(add_title)=input.trim().strip_prefix("add "){

            let todo= Todo{title: add_title.to_string(), done: false};
            println!("{:?}を追加しました。",todo.title);
            todos.push(todo);
            
        }else if input.trim()=="list"{
            println!("todo一覧");
            for to in &todos{

                println!("{:?}",to);
            }
        }else{
            println!("無効なコマンドです");
        }

        
    }
}
