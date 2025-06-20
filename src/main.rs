use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{env, fs};

struct Task {
    id: u32,
    title: String,
    done: bool,
}

impl Task {
    fn print_self(&self) {
        println!("Task {}: {}", self.id, self.title);
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
) {
    let f: File = File::open(file_name).expect("Error: No such file");
    let reader: BufReader<File> = BufReader::new(f);

    let mut line: String;
    let mut id: &str;
    let mut title: &str;
    let mut done: bool;
    let mut fields: Vec<&str>;
    let mut cur_task: Task;

    for line_result in reader.lines() {
        line = line_result.expect("Failed to read line");
        fields = line.split(",").collect();
        id = fields[0];
        if id == "id" || id == "MAX" {
            // skip first two lines of the csv
            continue;
        }
        title = fields[1];
        done = if fields[2] == "true" { true } else { false };

        cur_task = Task {
            id: id.parse().unwrap(),
            title: String::from(title),
            done,
        };
        if cur_task.done {
            complete_tasks.push(cur_task);
        } else {
            incomplete_tasks.push(cur_task);
        }
    }
}

fn get_max_task_id(file_name: &String) -> u32 {
    let f: File = File::open(file_name).expect("Error: No such file");
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Error: Could not read line");
    let fields: Vec<&str> = line.split(",").collect();
    if fields[0] != "MAX" {
        return 0;
    }
    fields[1].trim().parse().unwrap()
}

fn set_max_task_id(file_name: &String, new_max: u32) {
    let list_contents = fs::read_to_string(file_name).expect("Failed to read file");
    let mut lines: Vec<&str> = list_contents.lines().collect();

    let header_format = format!("MAX,{new_max}");
    let new_header: &str = &header_format;
    lines[0] = new_header;

    fs::write(file_name, lines.join("\n")).expect("Failed to update max task id");
}

// fn read_task_from_line(task: Task, line: &String) {}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage();
        return;
    }

    let command = &args[2];
    let file_name = &args[1];

    if command == "add" {
        let f: File = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file_name)
            .expect("Failed to open {file_name}");

        let mut writer: BufWriter<File> = BufWriter::new(f);
        let title = match args.get(3) {
            Some(t) => t,
            None => {
                print_usage();
                return;
            }
        };

        let id = get_max_task_id(file_name) + 1;
        let buf = String::from(format!("\n{id},{title},false"));
        writer
            .write_all(buf.as_bytes())
            .expect("Failed to write to file");

        set_max_task_id(file_name, id);
        println!("New task created with title {title}");
    } else if command == "list" {
        let mut complete_tasks: Vec<Task> = Vec::new();
        let mut incomplete_tasks: Vec<Task> = Vec::new();
        read_tasks(&mut incomplete_tasks, &mut complete_tasks, &file_name);

        println!("To-do:");
        for task in incomplete_tasks {
            task.print_self();
        }
        println!("------------------------------\n");
        println!("Complete:");
        for task in complete_tasks {
            task.print_self();
        }
    } else if command == "done" {
        let task_id = match args.get(3) {
            Some(t) => t,
            None => {
                print_usage();
                return;
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
                return;
            }
        }
        println!("Task not found. To see valid tasks, try: ./todo {file_name} list");
    } else if command == "init" {
        println!("file_name: {file_name}");
        let mut f: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name)
            .expect("Failed to create new tasklist");

        let init_contents = String::from("MAX,0\nid,description,completed");
        f.write(init_contents.as_bytes())
            .expect("Failed to write initial contents");
        println!("Initialized new tasklist {file_name}.");
    } else {
        print_usage();
    }
}
