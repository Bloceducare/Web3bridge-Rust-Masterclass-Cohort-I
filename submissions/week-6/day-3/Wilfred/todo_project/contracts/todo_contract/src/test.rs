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
fn update_todo() {
    let (env, client) = setup();

    // Step 1: Create a todo first
    let title = String::from_str(&env, "Go home!!!");
    let description = String::from_str(&env, "From Garage to the hostel");
    let todo = client.create_todo(&title, &description);

    // Step 2: Update the todo
    let new_title = String::from_str(&env, "Finish Homework");
    let new_description = String::from_str(&env, "Complete Rust assignment");

    let result = client.update_todo(&todo.id, &new_title, &new_description);

    // Step 3: Assert update returned true
    assert!(result);

    // Step 4: Fetch all todos and verify the update
    let all_todos = client.get_todos();
    let updated = all_todos.get(0).unwrap();

    assert_eq!(updated.title, new_title);
    assert_eq!(updated.description, new_description);
    assert_eq!(updated.id, todo.id);
    assert!(!updated.status); // Status should remain unchanged
}

#[test]
fn complete_todo() {
    let (env, client) = setup();

    // Step 1: Create a todo first
    let title = String::from_str(&env, "Go home!!!");
    let description = String::from_str(&env, "From Garage to the hostel");
    let todo = client.create_todo(&title, &description);

    // Step 2: Complete the todo (toggle status)
    let result = client.complete_todo(&todo.id);

    // Step 3: Assert the function returned true
    assert!(result);

    // Step 4: Fetch all todos and verify the status changed
    let all_todos = client.get_todos();
    let updated = all_todos.get(0).unwrap();

    assert!(updated.status); // Status should now be true

    // Step 5: Toggle again to make sure it can revert
    let result2 = client.complete_todo(&todo.id);
    assert!(result2);

    let all_todos2 = client.get_todos();
    let reverted = all_todos2.get(0).unwrap();

    assert!(!reverted.status); // Status should now be false again
}

