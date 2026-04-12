use std::{env};
use std::io::{self, Write};
use std::process::Command;

pub fn currdir() {
    let dir = env::current_dir().unwrap();
    let dirprint = dir.display();
    println!("{dirprint}");
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn git() {
    let output = Command::new("git").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e);
    });
    println!("{}", String::from_utf8_lossy(&output.stdout))
}

pub fn call_external(args: &[&str]) {
    if args.is_empty() {
        println!("Usage: enter a external process that should be called, eg: git");
        return;
    } else {
        let output = Command::new(args[0]).output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e);
        });
        println!("{}", String::from_utf8_lossy(&output.stdout))
    }
}