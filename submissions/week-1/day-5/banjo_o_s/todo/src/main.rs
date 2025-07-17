fn main() {
    let mut todo_list: VecTodo = VecTodo::new();
    let new_id: u32 = todo_list.id_counter;

    let todo: Todo = Todo::new(new_id, "Learn Rust".to_string(), "read the rust book".to_string());
    todo_list.add_todo(todo);
    println!("Todo List: {:?}", todo_list);

    let new_id: u32 = todo_list.id_counter;
    let another_todo: Todo = Todo::new(new_id, "gym".to_string(), "".to_string());
    todo_list.add_todo(another_todo);

    println!("after creating two todos{:?}", todo_list);

    // mark to completed
    // first get the todo from the list
    let found_todo: Option<(usize, &mut Todo)> = todo_list.get_todo(new_id);

    let (_, todo) = checker(found_todo);
    todo.mark_completed();


    println!("after completed::: {:?}", todo_list);

    // update the todo
    // get the todo from the list 
    let found_todo: Option<(usize, &mut Todo)> = todo_list.get_todo(new_id);
    let (_, todo) = checker(found_todo);

    todo.update_todo("gym".to_string(), "go to the gym".to_string());

    //after updating 
    println!("after updating todo: {:?}", todo_list);


    // edit the todo
    let found_todo: Option<(usize, &mut Todo)> = todo_list.get_todo(new_id);
    let(_, todo) = checker(found_todo);
    todo.edit_todo("go to the gym and lift weights".to_string());

    // after editing
    println!("after editing todo: {:?}", todo_list);

    // delete a todo
    todo_list.delete_todo(new_id);


    // after deleting
    println!("after deleted: {:?}", todo_list);

    let new_id: u32 = todo_list.id_counter;
    let third_todo: Todo = Todo::new(new_id, "tour".to_string(), "travel by road to 10 states".to_string());

    todo_list.add_todo(third_todo);

    println!("added a new todo after deletion ::: {:?}", todo_list);

    let optional_todo: Option<(usize, &mut Todo)> = todo_list.get_todo(new_id);
    let (_, todo) = checker(optional_todo);
    todo.mark_completed();


    println!("after the third todo is completed: {:?}", todo_list);


}

fn checker(found_todo: Option<(usize, &mut Todo)>) -> (usize, &mut Todo) {
    match found_todo {
        Some((i, v)) => (i, v),
        None => {
            println!("could not locate todo");
            return (0, found_todo.unwrap().1);
        },
    }
}

#[derive(Debug)]
struct VecTodo {
    todos: Vec<Todo>,
    id_counter: u32,
}


#[derive(Debug)]
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl VecTodo {

    fn new() -> Self {
        Self {
            todos: Vec::new(),
            id_counter: 1,
        }
    }

    fn add_todo(&mut self, todo: Todo) -> bool {
        // add the todo to the list
        self.todos.push(todo);

        // increment the id counter
        self.id_counter += 1;
        true
    }

    fn delete_todo(&mut self, id: u32) -> bool {
        // assert!(id != 0 && self.todos.len() >= id as usize, "invalid todo id provided");
        if id != 0 && id as usize > self.todos.len() {
            println!("invalid id provided = {}", id);
            return false;
        }
        self.todos.retain(|i | i.id != id);
        true
    }

    fn get_todo(&mut self, id: u32) -> Option<(usize, &mut Todo)> {
        for (index, todo) in self.todos.iter_mut().enumerate() {
            if todo.id == id {
                return Some((index, todo));
            }
        }
        None
    }
}

impl Todo {

    fn new(id: u32, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            completed: false,
        }
    }

    fn mark_completed(&mut self) {
        self.completed = true;
        return;
    }

    fn update_todo(&mut self, title: String, description: String) -> bool {
        self.title = title;
        self.description = description;
        true
    }

    // edit_todo is used to update the description of the todo
    fn edit_todo(&mut self, description: String) -> bool {
        self.description = description;
        true
    }
}