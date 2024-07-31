// src/todo.rs

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use chrono::{NaiveDate, ParseError};

#[derive(Debug)]
pub struct Todo {
    pub id: usize,
    pub description: String,
    pub priority: String,
    pub due_date: Option<NaiveDate>,
    pub categories: Vec<String>,
    pub done: bool,
}

impl Todo {
    pub fn new(id: usize, description: String, priority: String, due_date: Option<NaiveDate>, categories: Vec<String>) -> Todo {
        Todo {
            id,
            description,
            priority,
            due_date,
            categories,
            done: false,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
    }
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate, ParseError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
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
        if parts.len() == 5 {
            let due_date = if parts[2].is_empty() {
                None
            } else {
                Some(parse_date(parts[2]).unwrap())
            };
            let todo = Todo {
                id,
                description: parts[0].to_string(),
                priority: parts[1].to_string(),
                due_date,
                categories: parts[3].split('|').map(String::from).collect(),
                done: parts[4] == "done",
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
        let due_date = todo.due_date.map_or(String::new(), |d| d.to_string());
        let categories = todo.categories.join("|");
        writeln!(file, "{},{},{},{},{}", todo.description, todo.priority, due_date, categories, status)?;
    }
    Ok(())
}
