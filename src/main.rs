//imports start here
use crate::tools::regularcomms::*;
use crate::tools::misc::*;
use crate::tools::fileops::*;
use crate::tools::session_handling::*;
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
    pub mod session_handling;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{style_bold}{color_bright_cyan}═══════════════════════════════════════{color_reset}{style_reset}");
    println!("{style_bold}{color_bright_cyan}  {VER}{color_reset}{style_reset}"); // VER statt hardcoded!
    println!("{style_bold}{color_yellow}  ⚠️  DEVELOPMENT RELEASE ⚠️{color_reset}{style_reset}");
    println!("{style_bold}{color_bright_cyan}═══════════════════════════════════════{color_reset}{style_reset}");
    println!();
    let _ = login();       // erst login
    shell_loop()    // dann shell
}
