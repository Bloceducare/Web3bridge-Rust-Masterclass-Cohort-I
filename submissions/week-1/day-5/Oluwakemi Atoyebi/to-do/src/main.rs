#[derive(Debug, Clone)]

struct Lists {
    id: u32,
    title: String,
    description: String,
    is_completed: bool,
}

impl Lists {
    fn add_list(id: u32, title: String, description: String) -> Self {
      
        Self {
            id,
            title,
            description,
            is_completed: false,
        }
    }
}

fn main() {
    let mut next_id: u32 = 0;
    let mut todos: Vec<Lists> = Vec::new();
    
    let mut todo = Lists::add_list(next_id,
        String::from("Get an apartment"),
        String::from("Visit new agents"),
    );
    next_id += 1;

    // Update List

    let todo2 = Lists::add_list(next_id,
        String::from("Plan a vacation"),
        String::from("Get a verified tour company"),
    );
    next_id += 1;

    todos.push(todo.clone());
    todos.push(todo2.clone());

    println!("\n ToDo Lists:");
    for list in &todos {
        println!("{:#?}", list);
    }

}
