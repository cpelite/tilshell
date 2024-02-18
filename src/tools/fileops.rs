use std::io::{self, Write};
use std::fs::{File, OpenOptions, self};

use crate::main;
//use std::path::Path;

pub fn touch() {
    let mut user_input = String::new();
    println!("Enter the file path: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line!");
    let file_path = user_input.trim();

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
    main()
}

pub fn echo() {
    let mut user_input = String::new();
    println!("Enter the file path: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line!");
    let file_path = user_input.trim();

    let mut append_content = String::new();
    println!("Please enter what should be appended to file: ");
    let mut consecutive_empty_lines = 0;

    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");

        if buffer.trim().is_empty() {
            consecutive_empty_lines += 1;
        } else {
            consecutive_empty_lines = 0;
        }

        append_content.push_str(&buffer);

        // Stop input
        if consecutive_empty_lines == 1 {
            break;
        }
    }
    
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

pub fn cat() {
    let mut user_input = String::new();
    println!("Enter the file path: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line!");
    let file_path = user_input.trim();

    let file_contents = fs::read_to_string(file_path)
        .expect("Contents of file read successfully!");
    println!("Contents of {file_path} =\n{file_contents}");
    main();
}

pub fn mkdir() {
    let mut user_input = String::new();
    println!("Please enter the path where the new directory should be created: ");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read path!");

    let trimmed_path = user_input.trim();

    if trimmed_path.is_empty() {
        eprintln!("Error: Empty path provided. Please enter a valid path!");
        return;
    }

    if let Err(err) = fs::create_dir(trimmed_path) {
        eprintln!("Error: Failed to create directory. {}", err);
    } else {
        println!("Directory created successfully!");
    }
}

pub fn rmdir() {
    let mut user_input = String::new();
    println!("Please enter the path of the directory which shall be deleted.");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read path!");

    let trimmed_path = user_input.trim();

    if trimmed_path.is_empty() {
        eprintln!("Error: Empty path provided. Please enter a valid path!");
        return;
    }

    if let Err(err) = fs::remove_dir(trimmed_path) {
        eprintln!("Error: Failed to delete directory. {}", err);
    } else {
        println!("Directory deleted successfully!");
    }
}

pub fn renamedir() {
    let mut user_input = String::new();
    println!("Please enter the path of the directory which shall be renamed.");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read path!");

    println!("Please enter the new name for the directory. Please note that you must enter the full path, at least for now.");
    let mut newname = String::new();
    io::stdin()
        .read_line(&mut newname)
        .expect("Failed to read input!");

    let trimmed_path = user_input.trim();
    let newname = newname.trim();

    if trimmed_path.is_empty() {
        eprintln!("Error: Empty path provided. Please enter a valid path!");
        return;
    }

    if newname.is_empty() {
        eprintln!("Error: New name not provided. Please provide a new name.");
        return;
    }

    if let Err(err) = fs::rename(trimmed_path, newname) {
        eprintln!("Error: Failed to rename directory. {}", err);
    } else {
        println!("Directory renamed successfully!");
    }
}