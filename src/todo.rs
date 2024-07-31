// src/todo.rs

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub description: String,
    pub done: bool,
}

impl Todo {
    pub fn new(id: usize, description: String) -> Todo {
        Todo {
            id,
            description,
            done: false,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

pub fn load_todos(filename: &str) -> io::Result<Vec<Todo>> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            File::create(filename)?;
            File::open(filename)?
        }
        Err(e) => return Err(e),
    };

    let reader = BufReader::new(file);
    let mut todos = Vec::new();

    for (id, line) in reader.lines().enumerate() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let todo = Todo {
                id,
                description: parts[0].to_string(),
                done: parts[1] == "done",
            };
            todos.push(todo);
        }
    }

    Ok(todos)
}

pub fn save_todos(filename: &str, todos: &Vec<Todo>) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(filename)?;
    for todo in todos {
        let status = if todo.done { "done" } else { "not_done" };
        writeln!(file, "{},{}", todo.description, status)?;
    }
    Ok(())
}
