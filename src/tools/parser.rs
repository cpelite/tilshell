use crate::tools::fileops::*;
use crate::tools::misc::*;
use crate::tools::regularcomms::*;
use crate::tools::authentication::*;
use crate::tools::roles::*;
use inline_colorization::*;

pub fn parse_command(input: &str, role: &UserRole) {
    let mut parts = input.trim().split_whitespace();
    let command = parts.next().unwrap_or("");
    let args: Vec<&str> = parts.collect();

    match command {
        "" => return,
        "help" | "?" => help(role),
        "info" => info(),
        "exit" => exit(),
        "todo" => todo(),
        "cat" => cat(&args),
        "path" | "pwd" => currdir(),
        "chdir" | "cd" => chdir(&args),
        "dir" | "ls" => dir(),
        "clear" | "cls" => clear(),
        "swu" => switch_user(),
        "gshell" => {
            match role {
                UserRole::Guest => gshell_info(),
                _ => println!("{style_bold}{color_bright_red}Permission denied.{color_reset}{style_reset}"),
            }
        }
        "touch" | "echo" | "rm" | "mkdir" | "rmdir" | "rnd" => {
            match role {
                UserRole::Guest => println!("{style_bold}{color_bright_red}Permission denied.{color_reset}{style_reset}"),
                _ => match command {
                    "touch" => touch(&args),
                    "echo" => echo(&args),
                    "rm" => rmfile(&args),
                    "mkdir" => mkdir(&args),
                    "rmdir" => rmdir(&args),
                    "rnd" => renamedir(&args),
                    _ => unreachable!(),
                }
            }
        }
        "callext" => {
            match role {
                UserRole::Lynn => call_external(&args),
                _ => println!("{style_bold}{color_bright_red}Permission denied.{color_reset}{style_reset}"),
            }
        }
        "bshell_info" => {
            match role {
                UserRole::Bene => bshell_info(),
                _ => println!("{style_bold}{color_bright_red}Permission denied.{color_reset}{style_reset}"),
            }
        }
        "passwd" => {
            match role {
                UserRole::Lynn => { let _ = setup_pw(); }
                _ => println!("{style_bold}{color_bright_red}Permission denied.{color_reset}{style_reset}"),
            }
        }
        _ => println!("{style_bold}{color_bright_red}Unrecognized entry!{color_reset}{style_reset}"),
    }
}