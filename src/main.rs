// this will be a simple todo cli app
// start from parsing the command line arguments using clap library

use clap::{Arg, Command};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let matches = Command::new("Todo CLI")
        .version("0.1.0")
        .author("gcfbn")
        .about("A simple todo cli app written in Rust")
        .subcommand(
            Command::new("add")
                .about("Add a todo")
                .arg(Arg::new("todo").help("Todo to be added").required(true)),
        )
        .subcommand(Command::new("list").about("List all todos"))
        .subcommand(
            Command::new("done").about("Mark a todo as done").arg(
                Arg::new("todo")
                    .help("Todo to be marked as done")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("undone").about("Mark a todo as undone").arg(
                Arg::new("todo")
                    .help("Todo to be marked as undone")
                    .required(true),
            ),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a todo")
                .arg(Arg::new("todo").help("Todo to be removed").required(true)),
        )
        .get_matches();

    // get the current working directory
    let cwd = std::env::current_dir().unwrap();
    // get the todo file path
    let todo_file_path = cwd.join("todo.txt");

    // check if the todo file exists
    if !Path::new(&todo_file_path).exists() {
        // if not create the file
        fs::File::create(&todo_file_path).unwrap();
    }

    // check if the done file exists
    let done_file_path = cwd.join("done.txt");
    if !Path::new(&done_file_path).exists() {
        // if not create the file
        fs::File::create(&done_file_path).unwrap();
    }

    // check if the list subcommand is used
    if let Some(_) = matches.subcommand_matches("list") {
        // if yes, list all the todos
        let raw_todos = get_todos(&todo_file_path);
        let raw_dones = get_todos(&done_file_path);
        let todos: Vec<&String> = raw_todos.iter().filter(|t| t.trim().len() != 0).collect();
        let dones: Vec<&String> = raw_dones.iter().filter(|t| t.trim().len() != 0).collect();
        if todos.len() == 0 {
            println!("No todos found");
        } else {
            println!("Todos:");
            for (i, todo) in todos.iter().filter(|t| t.trim().len() != 0).enumerate() {
                println!("{}. {}", i + 1, todo);
            }
        }
        if dones.len() != 0 {
            println!("Done:");
            for (i, done) in dones.iter().enumerate() {
                println!("{}. {}", i + 1, done);
            }
        }
    }

    // check if the add subcommand is used
    if let Some(matches) = matches.subcommand_matches("add") {
        // if yes, add the todo
        let todo = matches.get_one::<String>("todo").unwrap();
        add_todo(&todo_file_path, todo);
        println!("Todo added successfully");
    }

    // check if the done subcommand is used
    if let Some(matches) = matches.subcommand_matches("done") {
        // if yes, mark the todo as done
        let todo = matches.get_one::<String>("todo").unwrap();
        mark_as_done(&todo_file_path, &done_file_path, todo);
    }

    // check if the undone subcommand is used
    if let Some(matches) = matches.subcommand_matches("undone") {
        // if yes, mark the todo as undone
        let todo = matches.get_one::<String>("todo").unwrap();
        mark_as_undone(&todo_file_path, &done_file_path, todo);
    }

    // check if the remove subcommand is used
    if let Some(matches) = matches.subcommand_matches("remove") {
        // if yes, remove the todo
        let todo = matches.get_one::<String>("todo").unwrap();
        remove_todo(&todo_file_path, todo);
    }
}

// function to get all the todos from the file
fn get_todos(file_path: &std::path::PathBuf) -> Vec<String> {
    // read the file
    let mut file = fs::File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // split the contents by new line
    contents.split("\n").map(|s| s.to_string()).collect()
}

// function to add a todo to the file
fn add_todo(file_path: &std::path::PathBuf, todo: &str) {
    // open the file in append mode
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    // write the todo to the file
    file.write_all(format!("{}\n", todo).as_bytes()).unwrap();
}

// function to mark a todo as done
fn mark_as_done(
    todo_file_path: &std::path::PathBuf,
    done_file_path: &std::path::PathBuf,
    todo: &str,
) {
    let mut todos = get_todos(todo_file_path);
    let mut dones = get_todos(done_file_path);

    // get the todo to be marked as done
    let todo_position = todos.iter().position(|t| t == todo);

    if todo_position.is_none() {
        println!("Todo not found");
        return;
    }

    todos.remove(todo_position.unwrap());
    dones.push(todo.to_string());

    fs::write(todo_file_path, todos.join("\n")).unwrap();
    add_todo(done_file_path, &todo);

    println!("Todo marked as done successfully");
}

// function to mark a todo as undone
fn mark_as_undone(
    todo_file_path: &std::path::PathBuf,
    done_file_path: &std::path::PathBuf,
    todo: &str,
) {
    let mut todos = get_todos(todo_file_path);
    let mut dones = get_todos(done_file_path);

    // get the todo to be marked as undone
    let todo_position = dones.iter().position(|t| t == todo);

    if todo_position.is_none() {
        println!("Todo not found");
        return;
    }

    dones.remove(todo_position.unwrap());
    todos.push(todo.to_string());

    fs::write(done_file_path, dones.join("\n")).unwrap();
    add_todo(todo_file_path, &todo);

    println!("Todo marked as undone successfully");
}

// function to remove a todo
fn remove_todo(file_path: &std::path::PathBuf, todo: &str) {
    let mut todos = get_todos(file_path);

    // get the todo to be removed
    let todo_position = todos.iter().position(|t| t == todo);

    if todo_position.is_none() {
        println!("Todo not found");
        return;
    }

    todos.remove(todo_position.unwrap());

    fs::write(file_path, todos.join("\n")).unwrap();

    println!("Todo removed successfully");
}
