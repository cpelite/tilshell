//imports start here
use std::io;
use std::io::Write;
use crate::tools::regularcomms::*;
use crate::tools::misc::*;
use crate::tools::fileops::*;
use inline_colorization::*;
//imports end here

//static strings start here
static VER: &str = "TilShell v2.1.0 | 2025-02-14";
static DEV: &str = "Dev: CPElite / ZlatinaDev";
//static strings end here

mod tools {
    pub mod fileops;
    pub mod regularcomms;
    pub mod misc;
}

pub fn parse_command(input: &str) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap_or(""); // Erster Teil ist der Befehl
    let args: Vec<&str> = parts.collect(); // Restliche Teile sind Argumente

    match command {
        "help" | "?" => help(),
        "info" => info(),
        "exit" => exit(),
        "todo" => todo(),
        "touch" => touch(&args),
        "echo" => echo(&args),
        "cat" => cat(&args),
        "path" => currdir(),
        "mkdir" => mkdir(&args),
        "rmdir" => rmdir(&args),
        "rnd" => renamedir(&args),
        "rm" => rmfile(&args),
        "chdir" => chdir(&args),
        "dir" => dir(),
        _ => println!("{style_bold}{color_bright_red}Unrecognized entry!{color_reset}{style_reset}"),
    }
}

fn main() {
    loop {
        print!("{style_bold}{color_bright_yellow}tsh>{color_reset}{style_reset} ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        parse_command(&input);
    }
}
