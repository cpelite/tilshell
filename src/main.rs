//imports start here
use std::io;
use crate::tools::regularcomms::*;
use crate::tools::misc::*;
use crate::tools::fileops::*;
use inline_colorization::*;
//imports end here

//static strings start here
static VER: &str = "TilShell v1.2.0 | 2024-02-18";
static DEV: &str = "Dev: CPElite / ZlatinaDev";
static SHD: &str = "[tsh1.2.0]";
//static strings end here

mod tools {
    pub mod fileops;
    pub mod regularcomms;
    pub mod misc;
}

fn main() {
    loop {
        println!("{color_magenta}{SHD}{color_reset}");
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
            "mkdir" => mkdir(),
            "rmdir" => rmdir(),
            "rnd" => renamedir(),
            _ => println!("{style_bold}{color_bright_red}Unrecognized entry!{color_reset}{style_reset}"),
        }
    }
}