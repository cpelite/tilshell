use std::io::{Write};
use std::fs::{File, OpenOptions, self};
use std::env;

use crate::main;
//use std::path::Path;

pub fn touch(args: &[&str]) {
    if args.is_empty() {
        println!("Usage: touch <file_path>");
        return;
    }
    let file_path = args[0];
    match File::create(file_path) {
        Ok(mut file) => {
            let content = "";
            match file.write_all(content.as_bytes()) {
                Ok(_) => println!("File created successfully at: {}", file_path),
                Err(e) => eprintln!("Failed to create file: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}

pub fn rmfile(args: &[&str]) {
    if args.is_empty() {
        println!("Usage: rm <file_path>");
        return;
    }
    let file_path = args[0];
    if let Err(e) = fs::remove_file(file_path) {
        eprintln!("Error: Failed to remove file. {}", e);
    } else {
        println!("Removed file {}", file_path);
    }
}

pub fn echo(args: &[&str]) {
    if args.len() < 2 {
        println!("Usage: echo <file_path> <text>");
        return;
    }
    let file_path = args[0];
    let content = args[1..].join(" ");
    match OpenOptions::new().append(true).create(true).open(file_path) {
        Ok(mut file) => {
            match file.write_all(content.as_bytes()) {
                Ok(_) => println!("Content appended successfully to: {}", file_path),
                Err(e) => eprintln!("Failed to write content to file: {}", e),
            }
        }
        Err(e) => eprintln!("Failed to open or create file: {}", e),
    }
}

pub fn cat(args: &[&str]) {
    if args.is_empty() {
        println!("Usage: cat <file_path>");
        return;
    }
    let file_path = args[0];
    match fs::read_to_string(file_path) {
        Ok(file_contents) => println!("Contents of {}:\n{}", file_path, file_contents),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}

pub fn mkdir(args: &[&str]) {
    if args.is_empty() {
        println!("Usage: mkdir <directory_path>");
        return;
    }
    let trimmed_path = args[0];
    if let Err(err) = fs::create_dir(trimmed_path) {
        eprintln!("Error: Failed to create directory. {}", err);
    } else {
        println!("Directory created successfully!");
    }
}

pub fn rmdir(args: &[&str]) {
    if args.is_empty() {
        println!("Usage: rmdir <directory_path>");
        return;
    }
    let trimmed_path = args[0];
    if let Err(err) = fs::remove_dir(trimmed_path) {
        eprintln!("Error: Failed to delete directory. {}", err);
    } else {
        println!("Directory deleted successfully!");
    }
}

// Works with parser.

pub fn renamedir(args: &[&str]) {
    if args.len() < 2 {
        eprintln!("Usage: rnd <current_path> <new_path>");
        return;
    }
    
    let trimmed_path = args[0];
    let newname = args[1];
    
    if let Err(err) = fs::rename(trimmed_path, newname) {
        eprintln!("Error: Failed to rename directory. {}", err);
    } else {
        println!("Directory renamed successfully!");
    }
}

pub fn chdir(args: &[&str]) {
    if args.is_empty() {
        println!("Usage: chdir <directory>");
        return;
    }
    let new_dir = args[0];
    match env::set_current_dir(new_dir) {
        Ok(_) => println!("Successfully changed working directory to: {}", new_dir),
        Err(e) => println!("Failed to change working directory: {}", e),
    }
}

pub fn dir() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    main()
}