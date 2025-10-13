use std::env;
use std::io::{self, Write};

pub fn currdir() {
    let dir = env::current_dir().unwrap();
    let dirprint = dir.display();
    println!("{dirprint}");
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}