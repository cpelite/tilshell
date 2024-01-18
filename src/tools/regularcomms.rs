use crate::main;
use crate::VER;
use crate::DEV;

pub fn help() {
    println!("The following commands are implemented currently:");
    println!("help - displays this command listing.");
    println!("info - displays information about shell.");
    println!("todo - opens the to-do list.");
    println!("exit - exits the shell.");
    println!("mkfile - creates a file.");
    println!("fileappend - appends content to a file.");
    main();
}

pub fn info() {
    println!("{VER}");
    println!("{DEV}");
    main();
}

pub fn exit() {
    println!("Exiting..");
}

pub fn todo() {
    println!("ToDo:");
    println!("Find a way to remove line breaks..");
    println!("Add a calculator.");
    println!("Adding what ever comes to my mind.");
    main();
}
