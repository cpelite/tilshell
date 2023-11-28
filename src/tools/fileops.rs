use std::io;
use std::fs::File;
use std::io::prelude;

use crate::main;
use crate::initandmain::return_to_nm;

pub fn fops_init() {
    println!("[ATTENTION!] Working with files is not properly implemented yet.");
    println!("Proceed with caution. Do you want to proceed? [y, n]");
    let mut initq: String = String::new();
    io::stdin().read_line(&mut initq);

    if initq.trim() == "y" {
        fops_main()
    }

    else if initq.trim() == "n" {
        println!("Returning to normal mode.");
        return_to_nm();
    }
}

fn fops_main() {
    println!("[~tsh0.1 - FOPS-Mode~]");
    let mut fopsusrinput: String = String::new();
    io::stdin().read_line(&mut fopsusrinput);

    if fopsusrinput.trim() == "help" {
        fops_help();
    } else if fopsusrinput.trim() == "?" {
        fops_help();
    }

    else if fopsusrinput.trim() == "nm" {
        return_to_nm();
    }
}

fn fops_help() {
    println!("The following commands are available in FOPS mode:");
    println!("help / ? - displays this command listing.");
    println!("nm - returns to normal mode.");
    fops_main()
}