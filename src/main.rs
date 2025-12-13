use std::fs::OpenOptions;
use std::io::Write;
use std::{io, option};

use std::fs;
fn main() {
    println!("Choose option: add / remove");

    let mut option = String::new();
    io::stdin().read_line(&mut option).unwrap();

    let option = option.trim();

    match option {
        "add" => {
            let mut input = String::new();
            println!("Enter todo:");
            io::stdin().read_line(&mut input).unwrap();
            save_todo(input.trim());
            show_todos();
        }
        "remove" => {
            let mut input = String::new();
            println!("Enter todo to remove:");
            io::stdin().read_line(&mut input).unwrap();
            remove_todo(input.trim());
            show_todos();
        }
        "update" => {
           
           let mut input=String::new();
              println!("Enter todo to update:");
                io::stdin().read_line(&mut input).unwrap();
            let to_update = input.trim().to_string();
            let mut new_input=String::new();
              println!("Enter new todo:");
                io::stdin().read_line(&mut new_input).unwrap();
            let new_todo = new_input.trim().to_string();
            update_todo(&to_update, &new_todo);
            show_todos();
        }
        _ => {
            println!("Invalid option");
        }
    }
}

fn save_todo(todo: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("todo.txt")
        .expect("Cannot open file");

    writeln!(file, "{}", todo).expect("Cannot write");
    
    
}
fn remove_todo(todo_to_remove: &str) {
    let content = fs::read_to_string("todo.txt").expect("Cannot read file");
    let todos: Vec<String> = content
        .lines()
        .filter(|line| line.trim() != todo_to_remove)
        .map(|s| s.to_string())
        .collect();

    fs::write("todo.txt", todos.join("\n")).expect("Cannot write file");
    println!("Todo removed (if it existed).");
}
fn update_todo(to_update: &str, new_todo: &str) {
    let content = fs::read_to_string("todo.txt").expect("Cannot read file");
    let todos: Vec<String> = content
        .lines()
        .filter(|line| line.trim() != to_update)
        .map(|s| s.to_string())
        .collect();
    let mut updated_todos = todos;
    updated_todos.push(new_todo.to_string());
    fs::write("todo.txt", updated_todos.join("\n")).expect("Cannot write file");
    println!("Todo updated (if it existed).");


}
fn show_todos() {
    match fs::read_to_string("todo.txt") {
        Ok(content) => {
            if content.trim().is_empty() {
                println!("No todos found.");
            } else {
                println!("\nCurrent TODOs:");
                for (i, line) in content.lines().enumerate() {
                    println!("{}. {}", i + 1, line);
                }
            }
        }
        Err(_) => println!("No todo file found."),
    }
}

