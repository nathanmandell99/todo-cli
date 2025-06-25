use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

impl Task {
    fn print_self(&self) {
        println!("Task {}: {}", self.id, self.description);
    }
}

fn print_usage() {
    println!("Usage:");
    println!("List tasks: todo <filename> list");
    println!("Add task: todo <filename> add <task name>");
    println!("Complete task: todo <filename> done <task id>");
    println!("Initialize new task list: todo <filename> init");
}

fn read_tasks(
    incomplete_tasks: &mut HashMap<u32, Task>,
    complete_tasks: &mut HashMap<u32, Task>,
    file_name: &str,
    create: bool,
) -> Result<(), Box<dyn Error>> {
    let f: File = match File::open(file_name) {
        Ok(file) => file,
        Err(_) if create => new_list(file_name)?,
        Err(err) => {
            return Err(err.into());
        }
    };
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(f);

    for line_result in reader.deserialize() {
        let cur_task: Task = line_result?;
        if cur_task.completed {
            complete_tasks.insert(cur_task.id, cur_task);
        } else {
            incomplete_tasks.insert(cur_task.id, cur_task);
        }
    }
    Ok(())
}

fn get_max_task_id(file_name: &str) -> Result<u32, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(file_name)?;
    let mut next_id = 1;

    for line_result in rdr.deserialize() {
        let task: Task = line_result?;
        next_id = next_id.max(task.id + 1);
    }
    Ok(next_id)
}

fn new_list(file_name: &str) -> Result<File, Box<dyn Error>> {
    let mut f: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)?;
    {
        let mut writer = csv::Writer::from_writer(&mut f);
        writer.write_record(&["id", "description", "completed"])?;
        // drop the writer here so that we don't get a double-free later
    }
    Ok(f)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        return Ok(());
    }

    let command: &str = &args[2];
    let file_name: &str = &args[1];

    match command {
        "add" => {
            let description = match args.get(3) {
                Some(t) => t,
                None => {
                    print_usage();
                    return Ok(());
                }
            };
            let task = Task {
                id: get_max_task_id(file_name)? + 1,
                description: description.clone(),
                completed: false,
            };

            let mut f: File = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_name)?;

            let mut writer = csv::Writer::from_writer(&mut f);
            writer.serialize(&task)?;

            println!("New task created with title {description}");
            Ok(())
        }
        "list" => {
            let mut complete_tasks: HashMap<u32, Task> = HashMap::new();
            let mut incomplete_tasks: HashMap<u32, Task> = HashMap::new();
            read_tasks(&mut incomplete_tasks, &mut complete_tasks, file_name, false)?;

            println!("To-do:");
            for (_id, task) in incomplete_tasks {
                task.print_self();
            }
            println!("------------------------------\n");
            println!("Complete:");
            for (_id, task) in complete_tasks {
                task.print_self();
            }
            Ok(())
        }
        "done" => {
            let task_id: u32 = match args.get(3) {
                Some(t) => match t.parse() {
                    Ok(task) => task,
                    Err(err) => {
                        return Err(err.into());
                    }
                },
                None => {
                    print_usage();
                    return Ok(());
                }
            };
            let mut complete_tasks: HashMap<u32, Task> = HashMap::new();
            let mut incomplete_tasks: HashMap<u32, Task> = HashMap::new();
            read_tasks(
                &mut incomplete_tasks,
                &mut complete_tasks,
                &file_name,
                false,
            )?;
            if let Some(task) = complete_tasks
                .get_mut(&task_id)
                .or_else(|| incomplete_tasks.get_mut(&task_id))
            {
                task.completed = !task.completed;
            } else {
                eprintln!("Task not found. To see valid tasks, try: ./todo {file_name} list");
            }

            let mut f: File = OpenOptions::new().write(true).open(file_name)?;

            let mut writer = csv::Writer::from_writer(&mut f);

            let mut combined_tasks: Vec<Task> = complete_tasks
                .drain()
                .chain(incomplete_tasks.drain())
                .map(|(_, t)| t)
                .collect();

            combined_tasks.sort_by_key(|t| t.id);

            for task in combined_tasks {
                writer.serialize(&task)?;
            }
            Ok(())
        }
        "init" => {
            new_list(file_name)?;
            Ok(())
        }
        _ => {
            print_usage();
            Ok(())
        }
    }
}
