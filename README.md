
# 📝 Todo CLI

A simple command-line task manager written in Rust. Tasks are stored in a CSV file with a basic format that includes a unique ID, description, and completion status.

## 📦 Features

- Initialize a new task list  
- Add tasks with a description  
- Mark tasks as complete/incomplete  
- List pending and completed tasks  

## 🚀 Usage

```bash
todo <filename> <command> [arguments]
```

### Commands

#### `init`
Initialize a new task list.

```bash
todo tasks.csv init
```

#### `add <task name>`
Add a new task to the list.

```bash
todo tasks.csv add "Buy groceries"
```

#### `list`
List all tasks, grouped into incomplete and complete.

```bash
todo tasks.csv list
```

#### `done <task id>`
Toggle the completion status of a task by its ID.

```bash
todo tasks.csv done 2
```

## 🗂 File Format

The task list is stored in a CSV file. The structure is:

```
MAX,<highest_task_id>
id,description,completed
1,Do laundry,false
2,Buy groceries,true
```

- The `MAX` line tracks the latest used ID.  
- Tasks start from the third line onward.  

## 🛠 Build

```bash
cargo build --release
```

## ✅ Example Workflow

```bash
todo tasks.csv init
todo tasks.csv add "Finish project"
todo tasks.csv list
todo tasks.csv done 1
```

## 📎 Notes

- No dependencies beyond the Rust standard library.  
- Errors are reported via `expect` with clear messages.
- This was mainly written to flex my Rust legs. For large task lists I imagine it is quite slow, given that most operations involve reading the entirety of the tasklist CSV file.
