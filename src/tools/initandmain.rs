use std::io;
use std::io::Read;
use crate::tools::fileops::fops_init;


pub fn init() {
    println!("ATTENTION! TilShell is still in a very state.");
    println!("Do you want to proceed? [y, n]");
    let mut initq: String = String::new();
    io::stdin().read_line(&mut initq);

    if initq.trim() == "y" {
        shmain()
    }

    else if initq.trim() == "n" {
        println!("Exiting.");
    }

}

pub fn return_to_nm() {
    shmain();
}

fn shmain() {
    println!("[~tsh0.1~]");
    let mut usrinput = String::new();
    io::stdin().read_line(&mut usrinput);

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
    println!("TilShell v0.1 | 20230910");
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
    println!("Implement access to the file system and allow reading of files.");
    shmain();
}