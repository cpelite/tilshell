use std::io;
use crate::tools::fileops::fops_init;

pub fn shmain() {
    println!("[~tsh0.2~]");
    let mut usrinput = String::new();
    io::stdin()
        .read_line(&mut usrinput)
        .expect("Failed to read user input!");

    if usrinput.trim() == "help" {
        help();
    } else if usrinput.trim() == "?" {
        help();
    }

    else if usrinput.trim() == "info" {
        info();
    }

    else if usrinput.trim() == "exit" {
        exit();
    }

    else if usrinput.trim() == "reload" {
        reload();
    }

    else if usrinput.trim() == "todo" {
        todo();
    }

    else if usrinput.trim() == "fops" {
        fops_init();
    }

    else {
        println!("Command not recognized!");
        shmain();
    }
}

fn help() {
    println!("The following commands are implemented currently:");
    println!("help - displays this command listing.");
    println!("info - displays information about shell.");
    println!("reload - reloads the shell.");
    println!("todo - opens the to-do list.");
    println!("exit - exits the shell.");
    println!("fops - enters File operations mode.");
    shmain();
}

fn info() {
    println!("TilShell v0.2 | 2024-01-18");
    println!("Dev: CPElite / ZlatinaDev");
    shmain();
}

fn exit() {
    println!("Exiting..");
}

fn reload() {
    shmain();
}

fn todo() {
    println!("ToDo:");
    println!("Find a way to remove line breaks..");
    println!("Add a calculator.");
    println!("Keep on improving file operations, so that FOPS-Mode can be discarded.");
    println!("Adding what ever comes to my mind.");
    shmain();
}