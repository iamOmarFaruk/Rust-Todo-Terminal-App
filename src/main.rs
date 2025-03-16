mod todo;
use std::process::exit;
use todo::{load_todos, create_todo, list_todos, update_todo, delete_todo, get_input, COLOR_GREEN, COLOR_RED};

/// Clears the screen
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}

/// Pauses the screen
fn pause_screen() {
    let _ = get_input("Press Enter to continue...");
}

/// Handles the main menu
fn menu() {
    let mut todos = load_todos();
    loop {
        clear_screen();
        println!("{}====== Todo App Menu ======{}", COLOR_GREEN, todo::COLOR_RESET);
        println!("1. Add Todo\n2. List Todos\n3. Update Todo\n4. Delete Todo\n5. Exit");
        println!("{}===========================\n", COLOR_GREEN);

        match get_input("Enter your choice: ").as_str() {
            "1" => { clear_screen(); create_todo(&mut todos); pause_screen(); }
            "2" => { clear_screen(); list_todos(&todos); pause_screen(); }
            "3" => { clear_screen(); update_todo(&mut todos); pause_screen(); }
            "4" => { clear_screen(); delete_todo(&mut todos); pause_screen(); }
            "5" => { clear_screen(); println!("{}Exiting... Goodbye!{}", COLOR_GREEN, todo::COLOR_RESET); exit(0); }
            _ => { println!("{}Invalid choice! Try again.{}", COLOR_RED, todo::COLOR_RESET); pause_screen(); }
        }
    }
}

fn main() {
    menu();
}
