use std::io;
use chrono::{DateTime, Local, Utc};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
enum Isdone {
    Done,
    Undone
}

#[derive(Serialize, Deserialize)]
struct Todo {
    title: String,
    done: Isdone,
    date: DateTime<Utc>
}

fn main() {
    println!("write your Todo : ");
    let mut todo_input = String::new();
    let mut is_done = String::new();
    io::stdin().read_line(&mut todo_input).expect("failed to read Todo_input");
    println!("Is it done ? ");
    io::stdin().read_line(&mut is_done).expect("failed to read is_done");

    let is_done_todo = match is_done.trim() {
        "yes" => Isdone::Done,
        "no" => Isdone::Undone,
        _ => {
            println!("you must say yes or no !");
            Isdone::Undone
        }
    };
    let todos: Todo = Todo {
        title: todo_input.trim().to_string(),
        done: is_done_todo,
        date: Local::now().with_timezone(&Utc)
    };
    let mut get_todos = if let Ok(content) = fs::read_to_string("text.json") {
        serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };
    get_todos.push(todos);
    let stringify = serde_json::to_string_pretty(&get_todos).unwrap();
    fs::write("text.json", stringify).expect("cannot write file !!")

}
