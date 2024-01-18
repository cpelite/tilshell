use crate::main;
use crate::VER;
use crate::DEV;

pub fn help() {
    println!("The following commands are implemented currently:");
    println!("help - displays this command listing.");
    println!("info - displays information about shell.");
    println!("todo - opens the to-do list.");
    println!("------------------------------------------");
    println!("exit - exits the shell.");
    println!("mkfile - creates a textfile.");
    println!("fileappend - appends content to a textfile.");
    println!("------------------------------------------");
    println!("cat - reads content from a textfile.");
    main();
}

pub fn info() {
    println!("{VER}");
    println!("{DEV}");
    main();
}

pub fn exit() {
    println!("Exiting TilShell.");
}

pub fn todo() {
    println!("ToDo:");
    println!("Find a way to remove line breaks..");
    println!("Add a calculator.");
    println!("Adding what ever comes to my mind.");
    main();
}
