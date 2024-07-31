# Rust TODO App

A simple command-line TODO application written in Rust. This project demonstrates basic file I/O, error handling, and the use of external crates such as `chrono` for date handling.

## Features

- Add TODO items with a description, priority, due date, and categories.
- List all TODO items.
- Mark TODO items as done.
- Save and load TODO items from a file.

## Requirements

- Rust (latest stable version)

## Setup

1. Clone the repository:

   ```sh
   git clone https://github.com/mukulhy/rust-todo-app.git
   cd rust-todo-app
   
## Build the project:
cargo build

## To add a todo
cargo run add "Description" "Priority" "Due Date" "Categories"

## To list all todo
cargo run list

## Mark a TODO as Done
cargo run done <id>
example :- cargo run done 1


