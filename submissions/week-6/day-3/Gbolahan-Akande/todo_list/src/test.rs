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
fn test_create() {
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

    client.delete(&id);

    let all_todo = client.get_todos();

    assert_eq!(all_todo.len(), 0);
}

#[test]
fn test_update() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Go Home!!!");
    let description = String:: from_str(&env, "From Garage to the Hostel");
    let id = 1_u32;

    client.create_todo(&title, &description);

    let title = String::from_str(&env, "Come to class");
    let description = String:: from_str(&env, "Come from hostel to Garage");

    client.update_todo(&id, &title, &description);

    let all_todo = client.get_todos();
    assert_eq!(all_todo.len(), id);
    let more = all_todo.first().unwrap();
    assert_eq!(more.title, title);
    assert_eq!(more.description, description);
}

#[test]
fn test_complete() {
    let (env, client) = setup();

    let title = String::from_str(&env, "Break Time");
    let description = String::from_str(&env, "Take a break for 15 minutes");
    let id = 1_u32;

    client.create_todo(&title, &description);
    client.complete(&id);

    let all_todo = client.get_todos();
    assert_eq!(all_todo.len(), 1);
    let the_first = all_todo.first().unwrap();
    assert!(the_first.status);
}
