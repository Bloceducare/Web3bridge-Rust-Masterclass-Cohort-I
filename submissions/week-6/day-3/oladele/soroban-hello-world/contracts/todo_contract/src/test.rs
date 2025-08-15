
#[cfg(test)]
mod test_todo_list{
    use crate::todo_list::{Todolist, Todo, TodolistClient};
use soroban_sdk::{vec, Env, String};
    fn setup() -> (Env, TodolistClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(Todolist, ());
    let client = TodolistClient::new(&env, &contract_id);

    (env, client)
}
#[test]
fn test() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Go home!!!");

    let description = String::from_str(&env, "From Garage to the hostel");

    let words = client.create_todo(&title, &description);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);
    assert_eq!(words.description, description);
    assert_eq!(words.title, title);
    assert_eq!(words.id, 1);
    assert!(!words.status);
}

#[test]
fn test_delete() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Go home!!!");

    let id = 1_u32;

    let description = String::from_str(&env, "From Garage to the hostel");

    client.create_todo(&title, &description);

    client.delete_todo(&id);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 0);
}

struct TitleAndDescriptions {
    title_1: String,
    title_2: String,
    title_3: String,
    title_4: String,
    title_5: String,
    description_1: String,
    description_2: String,
    description_3: String,
    description_4: String,
    description_5: String, 
}

fn create_many_titles_and_description(env: &Env) -> TitleAndDescriptions{
    TitleAndDescriptions {
        title_1: String::from_str(env, "first_title"),
        title_2: String::from_str(env, "second_title"),
        title_3: String::from_str(env, "third_title"),
        title_4: String::from_str(env, "fourth_title"),
        title_5: String::from_str(env, "fifth_title"),
        description_1: String::from_str(env, "first description"),
        description_2: String::from_str(env, "second description"),
        description_3: String::from_str(env, "third description"),
        description_4: String::from_str(env, "fourth description"),
        description_5: String::from_str(env, "fifth description"),
    }
}

#[test]
fn test_update_todo() {
    let (env, client) = setup();
    let t_d: TitleAndDescriptions = create_many_titles_and_description(&env);

    //cannot update a todo that does not exist
    let response = client.update_todo(&1, &t_d.title_1, &t_d.description_1);

    assert!(!response);

    client.create_todo(&t_d.title_1, &t_d.description_1);

    client.create_todo(&t_d.title_2, &t_d.description_2);

    client.complete_todo(&2);

    let todo: Option<Todo> = client.get_todo(&2);

    assert_eq!(todo.is_some(), true);
    assert!(todo.unwrap().status);

    client.update_todo(&1, &t_d.title_4, &t_d.description_3);
    let todo_1 = client.get_todo(&1).unwrap();

    assert_eq!(todo_1.title, t_d.title_4);
    assert_eq!(todo_1.description, t_d.description_3)
    
}
}

