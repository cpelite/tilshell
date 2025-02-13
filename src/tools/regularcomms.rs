use crate::main;
use crate::VER;
use crate::DEV;
use std::collections::BTreeMap;

pub fn help() {
    let mut commands = BTreeMap::new();
    commands.insert("help", "Show this command listing.");
    commands.insert("info", "Shows information about shell.");
    commands.insert("todo", "Shows the to-do list.");
    commands.insert("exit", "Closes the shell.");
    commands.insert("touch", "Creates a new file.");
    commands.insert("echo", "Appends content to file.");
    commands.insert("cat", "Reads from a text file.");
    commands.insert("path", "Shows current directory.");
    commands.insert("mkdir", "Creates a directory.");
    commands.insert("rmdir", "Removes a directory.");
    commands.insert("rnd", "Renames a directory.");
    commands.insert("rm", "Removes a file.");
    commands.insert("chdir", "Change the working directory.");
    commands.insert("dir", "Lists all files and folders in current directory.");

    println!("Available commands:");
    for (command, description) in &commands {
        println!("{:<20} - {}", command, description);

    }

    main()
}

pub fn info() {
    println!("{VER}");
    println!("{DEV}");
    main();
}

pub fn exit() {
    std::process::exit(0);
}

pub fn todo() {
    println!("ToDo:");
    println!("Find a way to remove line breaks..");
    println!("Add a calculator.");
    println!("Adding what ever comes to my mind.");
    println!("Improve working with files and paths.");
    main();
}