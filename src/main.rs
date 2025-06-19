// use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/* struct Task {
    id: u32,
    description: String,
    done: bool,
} */

fn print_usage() {
    println!("Usage:");
    println!("List tasks: todo list");
    println!("Add task: todo add <task name>");
    println!("Complete task: todo done <task id>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = &args[1];

    if command == "add" {
        println!("add task");
    } else if command == "list" {
        let f: File = File::open("list.txt").expect("Failed to open file.");
        let reader: BufReader<File> = BufReader::new(f);

        let mut line: String;
        let mut id: &str;
        let mut title: &str;
        let mut status: &str;
        let mut fields: Vec<&str>;

        for line_result in reader.lines() {
            line = line_result.expect("Failed to read line");
            fields = line.split(",").collect();
            id = fields[0];
            if id == "id" {
                // skip first line of the csv
                continue;
            }
            title = fields[1];
            status = if fields[2] == "true" {
                "complete"
            } else {
                "incomplete"
            };

            println!("Task {}: {} ({})", id, title, status);
        }
    } else if command == "done" {
        println!("list task as complete");
    } else {
        print_usage();
    }
}
