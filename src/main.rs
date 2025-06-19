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
        let mut reader: BufReader<File> = BufReader::new(f);
        let mut line: String = String::new();
        let mut len: usize = reader.read_line(&mut line).expect("Failed to read line");
        println!("read first line");

        let mut id: String;
        let mut title: String;
        let mut complete: String;

        while len > 0 {
            len = reader.read_line(&mut line).expect("Failed to read line");
            println!("read next line");
            for field in line.split(',') {
                println!("{field}");
            }
        }
    } else if command == "done" {
        println!("list task as complete");
    } else {
        print_usage();
    }
}
