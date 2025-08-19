#![cfg(test)]

use crate::todo_list::{Todolist, TodolistClient};

use soroban_sdk::{Env, String};

fn setup() -> (Env, TodolistClient<'static>) {
    let env = Env::default();
    let contract_id = env.register(Todolist, ());
    let client = TodolistClient::new(&env, &contract_id);

    (env, client)
}

#[test]
fn test_create_todo() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Workout");
    let description = String::from_str(&env, "Morning exercise routine");

    let title2 = String::from_str(&env, "Grocery Shopping");
    let description2 = String::from_str(&env, "Buy vegetables and fruits");

    let todo1 = client.create_todo(&title, &description);
    let todo2 = client.create_todo(&title2, &description2);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 2);
    assert_eq!(todo1.description, description);
    assert_eq!(todo1.title, title);
    assert_eq!(todo1.id, 1);
    assert!(!todo1.status);

    assert_eq!(todo2.description, description2);
    assert_eq!(todo2.title, title2);
    assert_eq!(todo2.id, 2);
    assert!(!todo2.status);
}

#[test]
fn test_update() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Workout");
    let description = String::from_str(&env, "Morning exercise routine");

    let todo = client.create_todo(&title, &description);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);
    assert_eq!(todo.title, title);
    assert_eq!(todo.description, description);

    let updated_title = String::from_str(&env, "Morning Workout");
    let updated_description = String::from_str(&env, "Nike Traning Club");

    let is_updated = client.update_todo(&todo.id, &updated_title, &updated_description);

    let all_todo = client.get_todos();

    assert!(is_updated);
    assert_eq!(all_todo.get(0).unwrap().id, 1);
    assert_eq!(all_todo.get(0).unwrap().title, updated_title);
    assert_eq!(all_todo.get(0).unwrap().description, updated_description);
}

#[test]
fn test_complete_todo() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Workout");
    let description = String::from_str(&env, "Morning exercise routine");

    let todo = client.create_todo(&title, &description);

    let is_completed = client.complete_todo(&todo.id);

    let all_todo = client.get_todos();

    assert!(is_completed);
    assert!(all_todo.get(0).unwrap().status);
}

#[test]
fn test_get_todos() {
    let (env, client) = setup();

    let title1 = String::from_str(&env, "Cooking");
    let description1 = String::from_str(&env, "Cook something delicious");

    let title2 = String::from_str(&env, "Grocery Shopping");
    let description2 = String::from_str(&env, "Buy vegetables and fruits");

    let all_todo = client.get_todos();
    assert_eq!(all_todo.len(), 0);

    client.create_todo(&title1, &description1);
    client.create_todo(&title2, &description2);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 2);
}

#[test]
fn test_get_todo() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Workout");
    let description = String::from_str(&env, "Morning exercise routine");

    let todo = client.create_todo(&title, &description);

    let todo = client.get_todo(&todo.id);

    assert_eq!(todo.id, todo.id);
    assert_eq!(todo.title, title);
    assert_eq!(todo.description, description);
    assert_eq!(todo.status, false);
}

#[test]
fn test_delete() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Workout");
    let description = String::from_str(&env, "Morning exercise routine");

    let title2 = String::from_str(&env, "Grocery Shopping");
    let description2 = String::from_str(&env, "Buy vegetables and fruits");

    client.create_todo(&title, &description);
    client.create_todo(&title2, &description2);

    let id = 1_u32;

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 2);

    client.delete_todo(&id);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);
}
