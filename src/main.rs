//default imports
use std::io;

//imports (regular commands)
use crate::tools::regularcomms::help;
use crate::tools::regularcomms::info;
use crate::tools::regularcomms::exit;
use crate::tools::regularcomms::todo;

//imports (file operations)
use crate::tools::fileops::mkfile;
use crate::tools::fileops::fileappend;
use crate::tools::fileops::cat;

//imports end here

//static strings start here
static VER: &str = "TilShell v1.0.1 | 2024-01-18";
static DEV: &str = "Dev: CPElite / ZlatinaDev";
//static strings end here

mod tools {
    pub mod fileops;
    pub mod regularcomms;
}

fn main() {
    println!("[tsh1.0]");
    let mut usrinput = String::new();
    io::stdin()
        .read_line(&mut usrinput)
        .expect("Failed to read user input!");

    match usrinput.as_str().trim() {
        "help" => help(),
        "?" => help(),
        "info" => info(),
        "exit" => exit(),
        "todo" => todo(),
        "mkfile" => mkfile(),
        "fileappend" => fileappend(),
        "cat" => cat(),
        _ => println!("Unrecognized entry!"),
    }
}