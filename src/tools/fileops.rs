use std::io::{self, Write};
use std::fs::{File, OpenOptions};
//use std::path::Path;

use crate::shellmain::shmain;

pub fn fops_init() {
    println!("[ATTENTION!] Working with files is not properly implemented yet.");
    println!("Proceed with caution. Do you want to proceed? [y, n]");
    let mut initq: String = String::new();
    io::stdin()
        .read_line(&mut initq)
        .expect("Failed to read user-input!");

    if initq.trim() == "y" {
        fops_main()
    }

    else if initq.trim() == "n" {
        println!("Returning to normal mode.");
        shmain();
    }
}

fn fops_main() {
    println!("[~tsh0.2 - FOPS-Mode~]");
    let mut fopsusrinput: String = String::new();
    io::stdin()
        .read_line(&mut fopsusrinput)
        .expect("Failed to read userinput!");

    if fopsusrinput.trim() == "help" {
        fops_help();

    } else if fopsusrinput.trim() == "?" {
        fops_help();
    }

    else if fopsusrinput.trim() == "mkfile" {
        fops_mkfile();
    }

    else if fopsusrinput.trim() == "fileappend" {
        fops_fileappend();
    }

    else if fopsusrinput.trim() == "nm" {
        shmain();
    }
}

fn fops_mkfile() {
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
    fops_main()
}

fn fops_fileappend() {
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
    fops_main();
}

fn fops_help() {
    println!("The following commands are available in FOPS mode:");
    println!("help / ? - displays this command listing.");
    println!("nm - returns to normal mode.");
    println!("mkfile - creates a new file.");
    println!("fileappend - appends a existing file.");
    fops_main()
}