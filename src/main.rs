use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use std::io::prelude::*;
use std::{io, process};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TodoList {
    todos: Vec<Todo>,
}

fn remove_todo_by_id(todo_list: &mut TodoList, id: u32) {
    if let Some(index) = todo_list.todos.iter().position(|todo| todo.id == id) {
        todo_list.todos.remove(index);
    }
}

fn main() -> std::result::Result<(), std::io::Error> {
    println!("Hi, this is a very original todo list.");

    if !(Path::new("todos.json").exists()) {
        let mut file = fs::File::create("todos.json")?;
        file.write_all(b"{\"todos\":[]}").unwrap();
    }

    let data = fs::read_to_string("todos.json").expect("Should have been able to read the file");
    let mut todos: TodoList = serde_json::from_str(&data)?;
    
    let mut answer: String = String::new();
    let stdin = io::stdin();
    
    println!("You currently have {} todos. What do you want to do?\n 1) View todos\n 2) Add a todo\n 3) Change completed status for a todo\n 4) Remove a todo\n 5) Exit", todos.todos.len());
    
    stdin.read_line(&mut answer)?; 
    match answer.trim() {
        "1" => {
            for todo in &todos.todos {
                println!("{} | {}.", if todo.completed { "✅" } else { "❌" }, todo.title);
            }
        }

        "2" => {
            println!("What do you want to add?");
            let mut todo_name = String::new();
            stdin.read_line(&mut todo_name)?;
            let todo_name = todo_name.trim();
            let todo = Todo {id: todos.todos.last().unwrap().id + 1 as u32, title: todo_name.to_string(), completed: false};
            todos.todos.push(todo);
            fs::write("todos.json", serde_json::to_string(&todos).unwrap())?;
        }
        "3" => {
            println!("Which one do you want to change? (0-{})", todos.todos.len()-1);
            for todo in &todos.todos {
                println!("ID: {} | {} | {}.", todo.id, if todo.completed { "✅" } else { "❌" }, todo.title);
            }
            let mut todo_id = String::new();
            stdin.read_line(&mut todo_id)?;
            let todo_id = todo_id.trim();
            let parsed_todo_id = todo_id.parse::<u32>().unwrap();
            for todo in &mut todos.todos {
                if todo.id == parsed_todo_id {
                    todo.completed = !todo.completed;
                    println!("\"{}\" is now {}.", todo.title, if todo.completed { "complete" } else { "incomplete" });
                }
            }
            
            fs::write("todos.json", serde_json::to_string(&todos).unwrap())?;
        }
        "4" => {
            println!("Which one do you want to remove? (0-{})", todos.todos.len()-1);
            for todo in &todos.todos {
                println!("ID: {} | {} | {}.", todo.id, if todo.completed { "✅" } else { "❌" }, todo.title);
            }
            let mut todo_id = String::new();
            stdin.read_line(&mut todo_id)?;
            let todo_id = todo_id.trim();
            let parsed_todo_id = todo_id.parse::<u32>().unwrap();
            
            remove_todo_by_id(&mut todos, parsed_todo_id);

            fs::write("todos.json", serde_json::to_string(&todos).unwrap())?;
        }

        "5" => {
            println!("Bye!");
            process::exit(0);
        }

        _ => {
            println!("I didn't understand. Please be more clear")
        }
    }

    Ok(())
}
