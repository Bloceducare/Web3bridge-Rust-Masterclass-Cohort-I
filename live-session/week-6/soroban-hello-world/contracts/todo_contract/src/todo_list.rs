use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: bool,
}

#[contracttype]
enum DataKey {
    Todos,
    NextID,
}

const TODOS: Symbol = symbol_short!("TOD0S");

const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

#[contract]
pub struct Todolist;

#[contractimpl]
impl Todolist {
    pub fn create_todo(env: Env, title: String, description: String) -> Todo {
        let mut todos = Self::get_todos(&env);

        let mut current_id = Self::get_id_enum(&env);

        let todo = Todo {
            id: current_id,
            title,
            description,
            status: false,
        };

        todos.push_back(todo.clone());

        env.storage().persistent().set(&DataKey::Todos, &todos);

        current_id += 1;

        env.storage().persistent().set(&DataKey::Todos, &current_id);

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
                env.storage().persistent().set(&DataKey::Todos, &todos);
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
                    env.storage().persistent().set(&DataKey::Todos, &todos);
                    return true;
                }
            }
        }
        false
    }

    pub fn delete_todo(env: Env, id: u32) -> bool {
        let mut todos = Self::get_todos(&env);

        if let Some(todo) = todos.iter().position(|i| i.id == id) {
            todos.remove(todo as u32);
            env.storage().persistent().set(&DataKey::Todos, &todos);
            return true;
        }

        false
    }

    pub fn update_todo2(env: Env, id: u32, title: String, description: String) -> bool {
        let mut todos = Self::get_todos(&env);

        for i in 0..todos.len() {
            if let Some(mut todo) = todos.get(i) {
                if todo.id == id {
                    todo.title = title;
                    todo.description = description;
                    todos.set(i, todo);
                    env.storage().persistent().set(&DataKey::Todos, &todos);

                    return true;
                }
            }
        }
        false
    }

    pub fn get_todos(env: &Env) -> Vec<Todo> {
        env.storage()
            .persistent()
            .get(&TODOS)
            .unwrap_or(Vec::new(env))
    }

    pub fn get_todos_enum(env: &Env) -> Vec<Todo> {
        env.storage()
            .persistent()
            .get(&DataKey::Todos)
            .unwrap_or(Vec::new(env))
    }
    pub fn get_id_enum(env: &Env) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::NextID)
            .unwrap_or(1)
    }
}
