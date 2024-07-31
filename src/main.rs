// src/main.rs

mod todo;

use std::env;
use todo::{Todo, load_todos, save_todos, parse_date};
use chrono::NaiveDate;

const FILENAME: &str = "todos.txt";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: todo_app <command> [args]");
        return;
    }

    let command = &args[1];
    let mut todos = load_todos(FILENAME).unwrap_or_else(|_| Vec::new());

    match command.as_str() {
        "add" => {
            if args.len() < 5 {
                eprintln!("Usage: todo_app add <description> <priority> <due_date> <categories>");
                return;
            }
            let description = args[2].clone();
            let priority = args[3].clone();
            let due_date = if args[4].is_empty() {
                None
            } else {
                Some(parse_date(&args[4]).expect("Invalid date format"))
            };
            let categories: Vec<String> = args[5].split('|').map(String::from).collect();
            let id = todos.len();
            let todo = Todo::new(id, description, priority, due_date, categories);
            todos.push(todo);
            save_todos(FILENAME, &todos).expect("Failed to save todos");
            println!("Todo added.");
        }
        "list" => {
            for todo in &todos {
                println!("{:?}", todo);
            }
        }
        "done" => {
            if args.len() < 3 {
                eprintln!("Usage: todo_app done <id>");
                return;
            }
            let id: usize = args[2].parse().expect("Invalid ID");
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                todo.mark_done();
                save_todos(FILENAME, &todos).expect("Failed to save todos");
                println!("Todo marked as done.");
            } else {
                println!("Todo not found.");
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
