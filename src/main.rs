use std::fs::OpenOptions;
use std::{io, option};
use std::io::Write;

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
        }
        "remove"=>{
              let mut input = String::new();
            println!("Enter todo to remove:");
            io::stdin().read_line(&mut input).unwrap();
            remove_todo(input.trim());
        }
        "update"=>{
            println!("Update feature not implemented yet.");
        }
        _=>{
            println!("Invalid option");
        }
      }
     
}

fn save_todo(todo:&str){
      let mut file= OpenOptions::new().create(true).append(true).open("todo.txt").expect("Cannot open file");
     
    writeln!(file, "{}", todo).expect("Cannot write");
    println!("{:?}",file);
}
fn remove_todo(todo_to_remove:&str){
    let content=fs::read_to_string("todo.txt").expect("Cannot read file");
     let todos: Vec<String> = content
        .lines()
        .filter(|line| line.trim() != todo_to_remove)
        .map(|s| s.to_string())
        .collect();

    fs::write("todo.txt", todos.join("\n"))
        .expect("Cannot write file");
     println!("Todo removed (if it existed).");
}