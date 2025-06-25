use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{File, OpenOptions};

/// Simple CSV-backed to-do list
#[derive(Parser)]
#[command(name = "todo", version, about)]
struct Cli {
    /// Path to the CSV file that stores the tasks
    #[arg(value_name = "FILE")]
    file_name: String,

    /// Sub-command to execute
    #[command(subcommand)]
    command: Command,

    /// Create list if it does not exist
    #[arg(short = 'c', long = "create")]
    create: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Add a new task with the given DESCRIPTION
    Add {
        #[arg(value_name = "DESCRIPTION")]
        description: String,
    },
    /// List all tasks
    List,
    /// Toggle completion status of the task with the given ID
    Toggle {
        #[arg(value_name = "ID")]
        task_id: u32,
    },
    /// Create an empty task list (CSV header row)
    Init,
}

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
    let mut next_id = 0;

    for line_result in rdr.deserialize() {
        let task: Task = line_result?;
        next_id = next_id.max(task.id);
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
    let cli = Cli::parse();

    match cli.command {
        Command::Add { description } => {
            let task = Task {
                id: get_max_task_id(&cli.file_name)? + 1,
                description: description.clone(),
                completed: false,
            };

            let mut f: File = OpenOptions::new()
                .write(true)
                .append(true)
                .create(cli.create)
                .open(&cli.file_name)?;

            let mut writer = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(&mut f);
            writer.serialize(&task)?;

            println!("New task created with title {description}");
            Ok(())
        }
        Command::List => {
            let mut complete_tasks: HashMap<u32, Task> = HashMap::new();
            let mut incomplete_tasks: HashMap<u32, Task> = HashMap::new();
            read_tasks(
                &mut incomplete_tasks,
                &mut complete_tasks,
                &cli.file_name,
                cli.create,
            )?;

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
        Command::Toggle { task_id } => {
            let mut complete_tasks: HashMap<u32, Task> = HashMap::new();
            let mut incomplete_tasks: HashMap<u32, Task> = HashMap::new();
            read_tasks(
                &mut incomplete_tasks,
                &mut complete_tasks,
                &cli.file_name,
                false,
            )?;
            if let Some(task) = complete_tasks
                .get_mut(&task_id)
                .or_else(|| incomplete_tasks.get_mut(&task_id))
            {
                task.completed = !task.completed;
            } else {
                eprintln!(
                    "Task not found. To see valid tasks, try: ./todo {} list",
                    cli.file_name
                );
            }

            let mut f: File = OpenOptions::new().write(true).open(&cli.file_name)?;

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
        Command::Init => {
            new_list(&cli.file_name)?;
            Ok(())
        }
    }
}
