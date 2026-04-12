use crate::tools::fileops::*;
use crate::tools::misc::*;
use crate::tools::regularcomms::*;
use inline_colorization::*;

pub fn parse_command(input: &str) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap_or(""); // first part = command
    let args: Vec<&str> = parts.collect(); // all other parts = arguments

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

pub fn parse_command_guest(input: &str) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();

    match command {
        "" => return,
        "help" | "?" => help_guest(),
        "info" => info(),
        "exit" => exit(),
        "todo" => todo(),
        "cat" => cat(&args),
        "path" | "pwd" => currdir(),
        "chdir" | "cd" => chdir(&args),
        "dir" | "ls" => dir(),
        "clear" | "cls" => clear(),
        "gshell" => gshell_info(),
        _ => println!("{style_bold}{color_bright_red}Unrecognized entry!{color_reset}{style_reset}"),
    }
}
