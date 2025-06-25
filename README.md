# 📝 Todo CLI

A small Rust command-line task manager that stores tasks in a plain CSV file.  
Each task has a unique numeric **id**, a **description**, and a **completed** flag.

## 📦 Features
- **Init** – create an empty task list (CSV with header)  
- **Add** – append a new task  
- **List** – show tasks grouped by status  
- **Toggle** – flip a task’s completion status  
- **`-c/--create` flag** – automatically create the CSV file if it doesn’t exist  

## 🚀 Usage
```bash
todo [-c|--create] <FILE> <COMMAND> [ARGS]
```

### Commands and Examples

#### `init`
Create an empty task list.
```bash
todo tasks.csv init
```

#### `add <DESCRIPTION>`
Add a new task.
```bash
todo tasks.csv add "Buy groceries"
```

#### `list`
Show all tasks grouped by status.
```bash
todo tasks.csv list
```

#### `toggle <ID>`
Toggle completion status of a task.
```bash
todo tasks.csv toggle 2
```

> **Tip:** The `-c` flag creates the file on first run:
> ```bash
> todo -c tasks.csv add "First task"
> ```

## 🗂 CSV Format
```csv
id,description,completed
1,Do laundry,false
2,Buy groceries,true
```
The program scans the file to determine the next id; no extra metadata lines are used.

## 🛠 Build & Run
```bash
cargo build --release
./target/release/todo tasks.csv init
```

### Dependencies
- `clap` – CLI argument parsing  
- `csv` – reading/writing CSV  
- `serde` (with `derive`) – (de)serializing tasks  

All dependencies are on crates.io; nothing else is required.

## ✅ Example Workflow
```bash
todo -c tasks.csv add "Finish project"
todo tasks.csv list
todo tasks.csv toggle 1
```

## 📎 Notes
- Performance is linear with file size because the whole CSV is read each time.  
- This project was mainly a learning exercise in Rust CLI ergonomics; contributions are welcome!
