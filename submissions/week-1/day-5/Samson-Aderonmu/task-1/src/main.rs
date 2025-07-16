
struct Todo {
    id: i32,
    title: String,
    description: String,
    is_completed: bool,
}


fn create_todo(id: i32, title: String, description: String) -> Todo {
    let new_todo = Todo {
        id,
        title,
        description,
        is_completed: false,
    };
    println!("✅ Created todo: {}", new_todo.title);
    new_todo
}


fn update_todo(todo: &mut Todo, new_title: String, new_description: String) {
    todo.title = new_title;
    todo.description = new_description;
    println!("🔄 Updated todo with ID: {}", todo.id);
}


fn delete_todo(todo: &mut Todo) {
    todo.title = "DELETED".to_string();
    todo.description = "This todo has been deleted".to_string();
    println!("🗑️ Deleted todo with ID: {}", todo.id);
}


fn edit_todo(todo: &mut Todo, new_title: String) {
    todo.title = new_title;
    println!("✏️ Edited todo with ID: {}", todo.id);
}


fn mark_completed(todo: &mut Todo) {
    todo.is_completed = true;
    println!("✅ Marked todo as completed: {}", todo.title);
}


fn display_todo(todo: &Todo) {
    let status = if todo.is_completed { "✅" } else { "⏳" };
    println!("{} [ID: {}] {}", status, todo.id, todo.title);
    println!("   Description: {}", todo.description);
    println!("   Completed: {}", todo.is_completed);
    println!("---");
}

fn main() {
    println!("🚀 Todo List");
    println!("============================");
    
    println!("\n1️⃣ Creating todos...");
    let mut todo1 = create_todo(1, "Learn Rust".to_string(), "Study Rust basics".to_string());
    let mut todo2 = create_todo(2, "Build Project".to_string(), "Create a web3 project".to_string());
    let mut todo3 = create_todo(3, "Write Code".to_string(), "Practice coding in Rust".to_string());
    
   
    println!("\n📋 Current Todos:");
    display_todo(&todo1);
    display_todo(&todo2);
    display_todo(&todo3);
    

    println!("\n2️⃣ Updating todo...");
    update_todo(&mut todo1, "Master Rust Programming".to_string(), "Complete Rust course".to_string());
    

    println!("\n3️⃣ Editing todo...");
    edit_todo(&mut todo2, "Build Web3 Application".to_string());
    

    println!("\n4️⃣ Marking todo as completed...");
    mark_completed(&mut todo3);
    

    println!("\n📋 Updated Todos:");
    display_todo(&todo1);
    display_todo(&todo2);
    display_todo(&todo3);
    

    println!("\n5️⃣ Deleting todo...");
    delete_todo(&mut todo2);
    

    println!("\n📋 Final Todo List:");
    display_todo(&todo1);
    display_todo(&todo2);
    display_todo(&todo3);
    
    println!("\n🎉 All operations completed!");
}
