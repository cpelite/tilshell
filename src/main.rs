//imports start here
use std::{env, io};
use std::io::Write;
use crate::tools::regularcomms::*;
use crate::tools::misc::*;
use crate::tools::fileops::*;
use inline_colorization::*;
//imports end here

//static strings start here
static VER: &str = "TilShell v3.0.2-dev | 2026-04-11";
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
        "" => return,
        "help" | "?" => help(),
        "info" => info(),
        "exit" => exit(),
        "todo" => todo(),
        "touch" => touch(&args),
        "echo" => echo(&args),
        "cat" => cat(&args),
        "path" | "pwd" => currdir(),
        "mkdir" => mkdir(&args),
        "rmdir" => rmdir(&args),
        "rnd" => renamedir(&args),
        "rm" => rmfile(&args),
        "chdir" | "cd" => chdir(&args),
        "dir" | "ls" => dir(),
        "clear" | "cls" => clear(),
        "git" => git(),
        "callext" => call_external(&args),
        _ => println!("{style_bold}{color_bright_red}Unrecognized entry!{color_reset}{style_reset}"),
    }
}

fn login() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Please enter your username");
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        if username.trim() == "lynnf" {
            println!("Welcome, Lynn.");
            // Shell-loop...
            break;
        } else {
            println!("Authentication failure!");
            println!("Restarting authentication!");
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{style_bold}{color_bright_cyan}═══════════════════════════════════════{color_reset}{style_reset}");
    println!("{style_bold}{color_bright_cyan}  TilShell v3.0.2-dev{color_reset}{style_reset}");
    println!("{style_bold}{color_yellow}  ⚠️  DEVELOPMENT RELEASE ⚠️{color_reset}{style_reset}");
    println!("{style_bold}{color_bright_cyan}═══════════════════════════════════════{color_reset}{style_reset}");
    println!();

    login()
}
