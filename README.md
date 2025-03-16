# 🦀 Rust Todo CLI App

A simple **terminal-based Todo Manager** built in **Rust** to practice **CRUD operations, file handling, and modular programming**. This project demonstrates working with **JSON for persistent storage**, **UUIDs for unique task IDs**, and **ANSI colors for a clean terminal UI**.

---

## 🚀 Features

✅ **Create, Read, Update, Delete (CRUD) operations**  
✅ **Data persistence** (Stores todos in `todos.json`)  
✅ **Formatted terminal output** (Neatly aligned with ANSI colors)  
✅ **Unique ID for todos** (Uses `uuid` for unique identifiers)  
✅ **User-friendly input handling**  
✅ **Timestamp tracking** (Created & Updated times with `chrono`)  
✅ **Modular code structure** (Divided into `main.rs` and `todo.rs`)  
✅ **Follows KISS, DRY, YAGNI & SOLID principles**  

---

## 📚 Learning Goals

This project was created to help **understand and practice**:
- **Rust File Handling** (Reading & writing JSON with `serde_json`)
- **User Input Handling in CLI**
- **Project Structure & Modular Code**
- **Date & Time Management (`chrono`)**
- **Generating Unique Identifiers (`uuid`)**
- **Rust Best Practices (KISS, DRY, SOLID, YAGNI)**

---

## 🛠 Installation & Setup

### 1️⃣ Clone the Repository
```sh
git clone https://github.com/your-username/rust-todo-cli.git
cd rust-todo-cli
```

### 2️⃣ Install Dependencies
Make sure you have **Rust** installed. If not, install it via [Rustup](https://rustup.rs/):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Then, navigate to the project directory and install dependencies:
```sh
cargo build
```

### 3️⃣ Run the Application
```sh
cargo run
```
This will launch the **interactive Todo Manager** in your terminal.

---

## 📜 Usage

### 📝 Adding a Todo
```sh
Enter title: Learn Rust
Enter description: Complete Rust book and practice exercises.
Enter status (default: Pending): In Progress
```

### 📋 Listing Todos
```sh
Todo List:
ID          : 3f3bc47c-3e91-4b2b-8d94-3cfa60e1b5f3
Title       : Learn Rust
Description : Complete Rust book and practice exercises.
Status      : In Progress
Created At  : 2025-03-16 12:30:00
Updated At  : 2025-03-16 12:30:00
------------------------------------
```

### 🔍 Searching for a Todo
```sh
Enter keyword to search: Rust
```

### ✏️ Updating a Todo
```sh
Enter the ID of the todo to update: 3f3bc47c-3e91-4b2b-8d94-3cfa60e1b5f3
Enter new title (leave empty to keep 'Learn Rust'):
Enter new description (leave empty to keep 'Complete Rust book and practice exercises.'):
Enter new status (leave empty to keep 'In Progress'): Completed
```

### ❌ Deleting a Todo
```sh
Enter the ID of the todo to delete: 3f3bc47c-3e91-4b2b-8d94-3cfa60e1b5f3
```

---

## 📂 Project Structure

```
rust-todo-cli/
├── Cargo.toml       # Project dependencies
└── src/
    ├── main.rs      # Handles menu and user interaction
    ├── todo.rs      # CRUD operations and file handling
```

---

## 🛠 Technologies Used

- **Rust** 🦀 (Systems programming)
- **Serde & Serde JSON** (JSON handling)
- **UUID** (Unique todo IDs)
- **Chrono** (Date & time formatting)
- **ANSI Colors** (Terminal UI)

---

## 🔥 Future Improvements

🔹 Add **status filtering** (e.g., list only "Pending" tasks)  
🔹 Implement **priority levels** (Low, Medium, High)  
🔹 Allow **bulk operations** (delete multiple todos at once)  
🔹 Export todos to **CSV or Markdown**  

---

## 📜 License

This project is open-source and available under the **MIT License**.

---

## ✨ Credits

Developed by [Omar Faruk](https://www.linkedin.com/in/omar-expert-webdeveloper/)  
📧 Contact: [omarbg.bd@gmail.com](mailto:omarbg.bd@gmail.com)

