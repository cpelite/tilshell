//default imports
use std::io;

//imports (regular commands)
use crate::tools::regularcomms::help;
use crate::tools::regularcomms::info;
use crate::tools::regularcomms::exit;
use crate::tools::regularcomms::todo;

//imports (misc.rs)
use crate::tools::misc::currdir;

//imports (file operations)
use crate::tools::fileops::touch;
use crate::tools::fileops::echo;
use crate::tools::fileops::cat;

//imports end here

//static strings start here
static VER: &str = "TilShell v1.1.0 | 2024-01-20";
static DEV: &str = "Dev: CPElite / ZlatinaDev";
//static strings end here

mod tools {
    pub mod fileops;
    pub mod regularcomms;
    pub mod misc;
}

fn main() {
    loop {
        println!("[tsh1.1.0]");
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
            "touch" => touch(),
            "echo" => echo(),
            "cat" => cat(),
            "path" => currdir(),
            _ => println!("Unrecognized entry!"),
        }
    }
}