#[derive(Debug)]
#[allow(dead_code)]
enum TodoPriority {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
enum TodoStatus {
    Active,
    Completed,
    Canceled
}

#[derive(Debug)]
struct TodoItem {
    title: String,
    priority: TodoPriority,
    status: TodoStatus,
}

impl TodoItem {
    fn new(title: String, priority: TodoPriority) -> Self {
        Self {
            title,
            priority,
            status: TodoStatus::Active,
        }
    }

    fn update_todo(&mut self, title: String, priority: TodoPriority) {
        self.title = title;
        self.priority = priority;
    }

    fn delete_todo(&mut self) {
        self.status = TodoStatus::Canceled;
    }

    fn mark_completed(&mut self) {
        self.status = TodoStatus::Completed;
    }
}



fn main() {
    let mut todo_item = TodoItem::new("Buy milk".to_string(), TodoPriority::Low);
    println!("{:?}", todo_item);
    todo_item.update_todo("Buy eggs".to_string(), TodoPriority::High);
    println!("updated todo item {:?}", todo_item);
    todo_item.delete_todo();
    println!("deleted todo item {:?}", todo_item);
    todo_item.mark_completed();
    println!("completed todo item{:?}", todo_item);
}
