use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};
use std::path::Path;
use serde_json;
use uuid::Uuid;
use chrono::{DateTime, Local};

/// ANSI color codes
pub const COLOR_RESET: &str = "\x1B[0m";
pub const COLOR_GREEN: &str = "\x1B[1;32m";
pub const COLOR_RED: &str = "\x1B[1;31m";
pub const COLOR_YELLOW: &str = "\x1B[1;33m";

const JSON_FILE: &str = "todos.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Formats a date string from RFC3339 to `YYYY-MM-DD HH:MM:SS`
pub fn format_datetime(datetime_str: &str) -> String {
    DateTime::parse_from_rfc3339(datetime_str)
        .map(|dt| dt.with_timezone(&Local).format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|_| datetime_str.to_string())
}

/// Prints a single Todo with aligned formatting
pub fn print_todo(todo: &Todo) {
    println!("{:<12}: {}", "ID", todo.id);
    println!("{:<12}: {}", "Title", todo.title);
    println!("{:<12}: {}", "Description", todo.description);
    println!("{:<12}: {}", "Status", todo.status);
    println!("{:<12}: {}", "Created At", format_datetime(&todo.created_at));
    println!("{:<12}: {}", "Updated At", format_datetime(&todo.updated_at));
    println!("-------------------------");
}

/// Loads all todos from the JSON file (returns an empty list if not found)
pub fn load_todos() -> Vec<Todo> {
    if !Path::new(JSON_FILE).exists() {
        return Vec::new();
    }
    File::open(JSON_FILE)
        .map(BufReader::new)
        .and_then(|reader| serde_json::from_reader(reader).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse JSON")))
        .unwrap_or_else(|_| Vec::new())
}

/// Saves todos to the JSON file
pub fn save_todos(todos: &[Todo]) -> io::Result<()> {
    OpenOptions::new().write(true).create(true).truncate(true).open(JSON_FILE)
        .and_then(|file| serde_json::to_writer_pretty(file, todos).map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
}

/// Reads user input with a prompt
pub fn get_input(prompt: &str) -> String {
    print!("{}{}", prompt, COLOR_RESET);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Creates a new todo item
pub fn create_todo(todos: &mut Vec<Todo>) {
    let title = get_input("Enter title: ");
    if title.is_empty() {
        println!("{}Title cannot be empty!{}", COLOR_RED, COLOR_RESET);
        return;
    }
    let description = get_input("Enter description: ");
    if description.is_empty() {
        println!("{}Description cannot be empty!{}", COLOR_RED, COLOR_RESET);
        return;
    }

    // ✅ Fixed Status Input Handling
    let status_input = get_input("Enter status (default: Pending): ");
    let status = if status_input.is_empty() { "Pending".to_string() } else { status_input };

    let now = Local::now().to_rfc3339();
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title,
        description,
        status,
        created_at: now.clone(),
        updated_at: now,
    };

    todos.push(todo);
    if save_todos(todos).is_ok() {
        println!("{}Todo added successfully!{}", COLOR_GREEN, COLOR_RESET);
    } else {
        println!("{}Failed to save todo!{}", COLOR_RED, COLOR_RESET);
    }
}

/// Lists all todo items
pub fn list_todos(todos: &[Todo]) {
    if todos.is_empty() {
        println!("{}No todo items available.{}", COLOR_YELLOW, COLOR_RESET);
    } else {
        println!("{}Todo List:{}", COLOR_GREEN, COLOR_RESET);
        for todo in todos {
            print_todo(todo);
        }
    }
}

/// Updates a todo item
pub fn update_todo(todos: &mut Vec<Todo>) {
    let id_input = get_input("Enter the ID of the todo to update: ");
    if let Some(todo) = todos.iter_mut().find(|t| t.id == id_input) {
        let new_title = get_input(&format!("Enter new title (leave empty to keep '{}'): ", todo.title));
        let new_description = get_input(&format!("Enter new description (leave empty to keep '{}'): ", todo.description));
        let new_status = get_input(&format!("Enter new status (leave empty to keep '{}'): ", todo.status));

        if !new_title.is_empty() { todo.title = new_title; }
        if !new_description.is_empty() { todo.description = new_description; }
        if !new_status.is_empty() { todo.status = new_status; }

        todo.updated_at = Local::now().to_rfc3339();

        if save_todos(todos).is_ok() {
            println!("{}Todo updated successfully!{}", COLOR_GREEN, COLOR_RESET);
        } else {
            println!("{}Failed to update todo!{}", COLOR_RED, COLOR_RESET);
        }
    } else {
        println!("{}Todo not found!{}", COLOR_RED, COLOR_RESET);
    }
}

/// Deletes a todo item
pub fn delete_todo(todos: &mut Vec<Todo>) {
    let id_input = get_input("Enter the ID of the todo to delete: ");
    if let Some(index) = todos.iter().position(|t| t.id == id_input) {
        todos.remove(index);
        if save_todos(todos).is_ok() {
            println!("{}Todo deleted successfully!{}", COLOR_GREEN, COLOR_RESET);
        } else {
            println!("{}Failed to delete todo!{}", COLOR_RED, COLOR_RESET);
        }
    } else {
        println!("{}Todo not found!{}", COLOR_RED, COLOR_RESET);
    }
}
