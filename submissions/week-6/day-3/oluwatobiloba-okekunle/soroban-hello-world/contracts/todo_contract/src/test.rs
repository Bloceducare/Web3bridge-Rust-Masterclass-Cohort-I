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
fn test_update_todo() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Watch Jumong");
    let description = String::from_str(&env, "Get Home and Watch Jumong on Amazon  Prime");
    
    let todo = client.create_todo(&title, &description);
    assert_eq!(todo.id, 1);
    
    let new_title = String::from_str(&env, "Read Manga");
    let new_description = String::from_str(&env, "Read Updated Manga");
    
    let result = client.update_todo(&todo.id, &new_title, &new_description);
    assert!(result);
    
    let todos = client.get_todos();
    assert_eq!(todos.len(), 1);
    
    let updated_todo = todos.get(0).unwrap();
    assert_eq!(updated_todo.title, new_title);
    assert_eq!(updated_todo.description, new_description);
    assert_eq!(updated_todo.id, 1);
    assert!(!updated_todo.status);
}

#[test]
fn test_update_todo2() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Watch Jumong");
    let description = String::from_str(&env, "Get Home and Watch Jumong on Amazon  Prime");
    
    let todo = client.create_todo(&title, &description);
    assert_eq!(todo.id, 1);
    
    let new_title = String::from_str(&env, "Read Manga 2");
    let new_description = String::from_str(&env, "Read Updated Manga 2");
    
    let result = client.update_todo2(&todo.id, &new_title, &new_description);
    assert!(result);
    
    let todos = client.get_todos();
    assert_eq!(todos.len(), 1);
    
    let updated_todo = todos.get(0).unwrap();
    assert_eq!(updated_todo.title, new_title);
    assert_eq!(updated_todo.description, new_description);
    assert_eq!(updated_todo.id, 1);
    assert!(!updated_todo.status);
}

#[test]
fn test_complete_todo() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Complete this task");
    let description = String::from_str(&env, "Task to be completed");
    
    let todo = client.create_todo(&title, &description);
    assert!(!todo.status);
    
    let result = client.complete_todo(&todo.id);
    assert!(result);
    
    let todos = client.get_todos();
    let completed_todo = todos.get(0).unwrap();
    assert!(completed_todo.status);
    
    let result = client.complete_todo(&todo.id);
    assert!(result);
    
    let todos = client.get_todos();
    let uncompleted_todo = todos.get(0).unwrap();
    assert!(!uncompleted_todo.status);
}

#[test]
fn test_multiple_todos() {
    let (env, client) = setup();

    let title1 = String::from_str(&env, "First Task");
    let description1 = String::from_str(&env, "First Description");
    let todo1 = client.create_todo(&title1, &description1);
    
    let title2 = String::from_str(&env, "Second Task");
    let description2 = String::from_str(&env, "Second Description");
    let todo2 = client.create_todo(&title2, &description2);
    
    let title3 = String::from_str(&env, "Third Task");
    let description3 = String::from_str(&env, "Third Description");
    let todo3 = client.create_todo(&title3, &description3);
    
    assert_eq!(todo1.id, 1);
    assert_eq!(todo2.id, 2);
    assert_eq!(todo3.id, 3);
    
    let todos = client.get_todos();
    assert_eq!(todos.len(), 3);
    
    let result = client.delete_todo(&2);
    assert!(result);
    
    let todos = client.get_todos();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos.get(0).unwrap().id, 1);
    assert_eq!(todos.get(1).unwrap().id, 3);
}

#[test]
fn test_update_nonexistent_todo() {
    let (env, client) = setup();
    
    let title = String::from_str(&env, "New Title");
    let description = String::from_str(&env, "New Description");
    
    let result = client.update_todo(&999, &title, &description);
    assert!(!result);
    
    let result = client.update_todo2(&999, &title, &description);
    assert!(!result);
}

#[test]
fn test_delete_nonexistent_todo() {
    let (_env, client) = setup();
    
    let result = client.delete_todo(&999);
    assert!(!result);
}

#[test]
fn test_complete_nonexistent_todo() {
    let (_env, client) = setup();
    
    let result = client.complete_todo(&999);
    assert!(!result);
}

#[test]
fn test_empty_todos_list() {
    let (_env, client) = setup();
    
    let todos = client.get_todos();
    assert_eq!(todos.len(), 0);
}

#[test]
fn test_todo_persistence() {
    let (env, client) = setup();

    let title1 = String::from_str(&env, "Persistent Task");
    let description1 = String::from_str(&env, "This should persist");
    client.create_todo(&title1, &description1);
    
    let todos_before = client.get_todos();
    assert_eq!(todos_before.len(), 1);
    
    let title2 = String::from_str(&env, "Another Task");
    let description2 = String::from_str(&env, "Second persistent task");
    client.create_todo(&title2, &description2);
    
    let todos_after = client.get_todos();
    assert_eq!(todos_after.len(), 2);
    assert_eq!(todos_after.get(0).unwrap().title, title1);
    assert_eq!(todos_after.get(1).unwrap().title, title2);
}

