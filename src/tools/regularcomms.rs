use crate::VER;
use crate::DEV;
use std::collections::BTreeMap;
use crate::tools::roles::*;
use crate::main;

pub fn help(role: &UserRole) {
    let mut commands = BTreeMap::new();
    // für alle
    commands.insert("help", "Show this command listing.");
    commands.insert("info", "Shows information about shell.");
    commands.insert("todo", "Shows the to-do list.");
    commands.insert("exit", "Closes the shell.");
    commands.insert("cat", "Reads from a text file.");
    commands.insert("path", "Shows current directory.");
    commands.insert("chdir", "Change the working directory.");
    commands.insert("dir", "Lists all files and folders in current directory.");
    commands.insert("clear", "Clears screen.");
    commands.insert("swu", "Switches User");

    match role {
        UserRole::Guest => {
            commands.insert("gshell", "Displays information about the guest shell.");
        }
        UserRole::Bene => {
            commands.insert("touch", "Creates a new file.");
            commands.insert("echo", "Appends content to file.");
            commands.insert("rm", "Removes a file.");
            commands.insert("mkdir", "Creates a directory.");
            commands.insert("rmdir", "Removes a directory.");
            commands.insert("rnd", "Renames a directory.");
            commands.insert("git", "Calls git.");
            commands.insert("bshell_info", "Displays information about Bene-Subshell.");
        }
        UserRole::Lynn => {
            commands.insert("touch", "Creates a new file.");
            commands.insert("echo", "Appends content to file.");
            commands.insert("rm", "Removes a file.");
            commands.insert("mkdir", "Creates a directory.");
            commands.insert("rmdir", "Removes a directory.");
            commands.insert("rnd", "Renames a directory.");
            commands.insert("git", "Calls git.");
            commands.insert("callext", "Calls external commands.");
            commands.insert("passwd", "Set up password.");
        }
    }

    println!("Available commands:");
    for (command, description) in &commands {
        println!("{:<20} - {}", command, description);
    }
}

pub fn info() {
    println!("{VER}");
    println!("{DEV}");
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
}

pub fn gshell_info() {
    println!("You are currently using the Sub-Shell for Guests.");
    println!("This shell is a shell with limited features, which only allows read operations.");
}

pub fn bshell_info() {
    println!("Bene-Subshell is a power user shell, whilst Lynn-Subshell allows calls to external commands.");
}

pub fn switch_user() {
    main();
}