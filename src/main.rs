use serde::Deserialize;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{env, fs};

#[derive(Debug, Deserialize)]
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
    incomplete_tasks: &mut Vec<Task>,
    complete_tasks: &mut Vec<Task>,
    file_name: &String,
    create: bool,
) -> Result<(), Box<dyn Error>> {
    let f: File = match File::open(file_name) {
        Ok(file) => file,
        Err(_) if create => new_list(file_name)?,
        Err(err) => {
            return Err(err.into());
        }
    };
    let mut reader = csv::Reader::from_reader(f);

    for line_result in reader.deserialize() {
        let cur_task: Task = line_result?;
        if cur_task.completed {
            complete_tasks.push(cur_task);
        } else {
            incomplete_tasks.push(cur_task);
        }
    }
    Ok(())
}

fn get_max_task_id(file_name: &String) -> Result<u32, Box<dyn Error>> {
    let f: File = File::open(file_name)?;
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let fields: Vec<&str> = line.split(",").collect();
    if fields[0] != "MAX" {
        return Ok(0);
    }
    Ok(fields[1].trim().parse()?)
}

fn set_max_task_id(file_name: &String, new_max: u32) -> Result<(), Box<dyn Error>> {
    let list_contents = fs::read_to_string(file_name)?;
    let mut lines: Vec<&str> = list_contents.lines().collect();

    let header_format = format!("MAX,{new_max}");
    let new_header: &str = &header_format;
    lines[0] = new_header;

    fs::write(file_name, lines.join("\n"))?;
    Ok(())
}

fn new_list(file_name: &String) -> Result<File, Box<dyn Error>> {
    let mut f: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)?;

    let init_contents = String::from("id,description,completed");
    f.write(init_contents.as_bytes())?;
    println!("Initialized new tasklist {file_name}.");
    Ok(f)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        return Ok(());
    }

    let command: &str = &args[2];
    let file_name = &args[1];

    match command {
        "add" => {
            let f: File = OpenOptions::new()
                .write(true)
                .append(true)
                .open(file_name)?;

            let mut writer: BufWriter<File> = BufWriter::new(f);
            let title = match args.get(3) {
                Some(t) => t,
                None => {
                    print_usage();
                    return Ok(());
                }
            };

            let id = get_max_task_id(file_name)? + 1;
            let buf = String::from(format!("\n{id},{title},false"));
            writer.write_all(buf.as_bytes())?;

            set_max_task_id(file_name, id)?;
            println!("New task created with title {title}");
            Ok(())
        }
        "list" => {
            let mut complete_tasks: Vec<Task> = Vec::new();
            let mut incomplete_tasks: Vec<Task> = Vec::new();
            read_tasks(
                &mut incomplete_tasks,
                &mut complete_tasks,
                &file_name,
                false,
            )?;

            println!("To-do:");
            for task in incomplete_tasks {
                task.print_self();
            }
            println!("------------------------------\n");
            println!("Complete:");
            for task in complete_tasks {
                task.print_self();
            }
            Ok(())
        }
        "done" => {
            let task_id = match args.get(3) {
                Some(t) => t,
                None => {
                    print_usage();
                    return Ok(());
                }
            };
            let list_contents = fs::read_to_string(file_name).expect("Failed to read file");
            let mut lines: Vec<&str> = list_contents.lines().collect();

            for i in 2..lines.len() {
                let mut fields: Vec<&str> = lines[i].split(',').collect();
                let cur_id = fields[0];
                if cur_id == task_id {
                    if fields[2] == "false" {
                        fields[2] = "true";
                    } else {
                        fields[2] = "false";
                    }
                    let new_line = fields.join(",");
                    lines[i] = &new_line;
                    fs::write(file_name, lines.join("\n")).expect("Failed to mark task done");
                    return Ok(());
                }
            }
            println!("Task not found. To see valid tasks, try: ./todo {file_name} list");
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
