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

//imports end here

//static strings start here
static VER: &str = "TilShell v1.0-dev | 2024-01-18";
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

    else if usrinput.trim() == "todo" {
        todo();
    }

    else if usrinput.trim() == "mkfile" {
        mkfile()
    }

    else if usrinput.trim() == "fileappend" {
        fileappend()
    }

    else {
        println!("Command not recognized!");
        main();
    }

}