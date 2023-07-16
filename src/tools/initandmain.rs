use std::io;
use std::io::Read;

pub fn init() {
    println!("Welcome to TilShell!");
    println!("Press a key to continue.");
    let buffer = &mut [0u8];
    io::stdin().read_exact(buffer).unwrap();
    shmain()
}

fn shmain() {
    println!("[user@tilsh]");
    let mut usrinput = String::new();
    io::stdin().read_line(&mut usrinput);

    if usrinput.trim() == "help" {
        help();
    }

    else if usrinput.trim() == "info" {
        info();
    }

    else if usrinput.trim() == "exit" {
        exit();
    }

    else {
        println!("Command not recognized!)")
    }
}

fn help() {
    println!("The following commands are implemented currently:");
    println!("help - displays this command listing.");
    println!("info - displays information about shell.");
    println!("exit - exits the shell.");
    shmain();
}

fn info() {
    println!("TilShell v0.1");
    println!("Dev: CPElite / ZlatinaDev");
    shmain();
}

fn exit() {
    println!("Exiting..");
}