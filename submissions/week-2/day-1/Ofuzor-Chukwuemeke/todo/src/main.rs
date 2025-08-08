
struct Todo{
    name:String,
    id:i8,
    task:String
}


struct Todos{
    todos:Vec<Todo>
}

impl Todos{
    fn new()->Todos{
        Todos{
            todos:Vec::new()
        };
    }

    fn create_todos(&self, todo:Todo){
        self.todos.push(todo);
    }



}


fn main() {
    println!("Hello, world!");
}
