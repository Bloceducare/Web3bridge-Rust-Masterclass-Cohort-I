use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: bool,
}

const TODOS: Symbol = symbol_short!("TOD0S");
const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

#[contract]
pub struct Todolist;

#[contractimpl]
impl Todolist {
    pub fn create_todo(env: Env, title: String, description: String) -> Todo {
        let mut todos = Self::get_todos(&env);
        let mut current_id = env.storage().persistent().get(&NEXT_ID).unwrap_or(1);

        let todo = Todo {
            id: current_id,
            title,
            description,
            status: false,
        };

        todos.push_back(todo.clone());

        env.storage().persistent().set(&TODOS, &todos);

        current_id += 1;

        env.storage().persistent().set(&NEXT_ID, &current_id);

        todo
    }

    pub fn update_todo(env: Env, id: u32, title: String, description: String) -> bool {
        let mut todos = Self::get_todos(&env);

        for i in 0..todos.len() {
            let mut updated = todos.get(i).unwrap();
            if updated.id == id {
                updated.title = title;
                updated.description = description;
                todos.set(i, updated);
                env.storage().persistent().set(&TODOS, &todos);
                return true;
            }
        }
        false
    }

    pub fn complete_todo(env: Env, id: u32) -> bool {
        let mut todos = Self::get_todos(&env);
        for i in 0..todos.len() {
            if let Some(mut todo) = todos.get(i) {
                if todo.id == id {
                    todo.status = !todo.status;
                    todos.set(i, todo);
                    env.storage().persistent().set(&TODOS, &todos);
                    return true;
                }
            }
        }
        false
    }

    pub fn delete_todo(env: Env, id: u32) -> bool {
        let mut todos = Self::get_todos(&env);

        if let Some(todo_by_index) = todos.iter().position(|todo| todo.id == id) {
            todos.remove(todo_by_index as u32);
            env.storage().persistent().set(&TODOS, &todos);
            return true;
        }
        false
    }

    pub fn get_todo(env: &Env, id: u32) -> Todo {
        let todos = Self::get_todos(env);
        todos.into_iter().find(|todo| todo.id == id).unwrap()
    }

    pub fn get_todos(env: &Env) -> Vec<Todo> {
        env.storage()
            .persistent()
            .get(&TODOS)
            .unwrap_or(Vec::new(env))
    }
}
