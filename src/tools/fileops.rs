use std::io::{self, Write};
use std::fs::{File, OpenOptions};

use crate::main;
//use std::path::Path;

pub fn mkfile() {
    let mut user_input = String::new();
    println!("Enter the file path: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line!");
    let file_path = user_input.trim();

    match File::create(file_path) {
        Ok(mut file) => {
            let content = "File successfully created!";
            match file.write_all(content.as_bytes()) {
                Ok(_) => println!("File created successfully at: {}", file_path),
                Err(e) => eprintln!("Failed to write content to file: {}", e),
            }
        }

        Err(e) => eprintln!("Failed to create file: {}", e),
    }
    main()
}

pub fn fileappend() {
    let mut user_input = String::new();
    println!("Enter the file path: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line!");
    let file_path = user_input.trim();

    let mut append_content = String::new();
    println!("Please enter what should be appended to file: ");
    io::stdin()
        .read_line(&mut append_content)
        .expect("Failed to read file!");

    match OpenOptions::new().append(true).create(true).open(file_path) {
        Ok(mut file) => {
            let content = &append_content;
            match file.write_all(content.as_bytes()) {
                Ok(_) => println!("Content appended successfully to: {}", file_path),
                Err(e) => eprintln!("Failed to write content to file: {}", e),
            }
        }

        Err(e) => eprintln!("Failed to open or create file: {}", e),
    }
    main();
}

/* fn fops_help() {
    println!("The following commands are available in FOPS mode:");
    println!("help / ? - displays this command listing.");
    println!("nm - returns to normal mode.");
    println!("mkfile - creates a new file.");
    println!("fileappend - appends a existing file.");
    fops_main()
} */