// todo 结构体
pub struct MyTodo {
    pub todo_name: String,
    pub is_finish: bool,
}

impl MyTodo {
    pub fn new(s1: String) -> Self {
        MyTodo {
            todo_name: s1,
            is_finish: false,
        }
    }
}
