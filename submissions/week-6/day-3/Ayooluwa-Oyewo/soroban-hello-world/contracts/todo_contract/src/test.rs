#![cfg(test)]

use crate::todo_list::{Todolist, TodolistClient};

use super::*;
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

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);

    client.delete_todo(&id);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 0);
}

#[test]
fn test_update() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Go home!!!");
    let description = String::from_str(&env, "From Garage to the hostel");

    client.create_todo(&title, &description);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);

    let updated_title = String::from_str(&env, "Go home!!! (updated)");
    let updated_description = String::from_str(&env, "From Garage to the hostel (updated)");

    client.update_todo(&1, &updated_title, &updated_description);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);
    assert_eq!(all_todo.get(0).unwrap().title, updated_title);
    assert_eq!(all_todo.get(0).unwrap().description, updated_description);
}

#[test]
fn test_complete() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Go home!!!");
    let description = String::from_str(&env, "From Garage to the hostel");

    client.create_todo(&title, &description);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);

    client.complete_todo(&1);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);
    assert!(all_todo.get(0).unwrap().status);
}

#[test]
fn test_update2() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Go home!!!");
    let description = String::from_str(&env, "From Garage to the hostel");

    client.create_todo(&title, &description);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);

    let updated_title = String::from_str(&env, "Go home!!! (updated)");
    let updated_description = String::from_str(&env, "From Garage to the hostel (updated)");

    client.update_todo(&1, &updated_title, &updated_description);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);
    assert_eq!(all_todo.get(0).unwrap().title, updated_title);
    assert_eq!(all_todo.get(0).unwrap().description, updated_description);
}

#[test]
fn test_create() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Go home!!!");
    let description = String::from_str(&env, "From Garage to the hostel");

    client.create_todo(&title, &description);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 1);
    assert_eq!(all_todo.get(0).unwrap().title, title);
    assert_eq!(all_todo.get(0).unwrap().description, description);
}

#[test]
fn test_get_todos() {
    let (env, client) = setup();

    let title1 = String::from_str(&env, "Go home!!!");
    let description1 = String::from_str(&env, "From Garage to the hostel");

    client.create_todo(&title1, &description1);

    let title2 = String::from_str(&env, "Buy groceries");
    let description2 = String::from_str(&env, "Milk, Bread, Eggs");

    client.create_todo(&title2, &description2);

    let all_todos = client.get_todos();

    assert_eq!(all_todos.len(), 2);
    assert_eq!(all_todos.get(0).unwrap().title, title1);
    assert_eq!(all_todos.get(0).unwrap().description, description1);
    assert_eq!(all_todos.get(1).unwrap().title, title2);
    assert_eq!(all_todos.get(1).unwrap().description, description2);
}