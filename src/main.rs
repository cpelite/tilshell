//imports start here
use crate::tools::session_handling::*;
use crate::tools::parser::*;
use inline_colorization::*;
//imports end here

//static strings start here
static VER: &str = "TilShell v4.0.0-dev | 2026-04-12";
static DEV: &str = "Dev: CPElite / ZlatinaDev";
//static strings end here

mod tools {
    pub mod fileops;
    pub mod regularcomms;
    pub mod misc;
    pub mod session_handling;
    pub mod parser;
    pub mod authentication;
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{style_bold}{color_bright_cyan}═══════════════════════════════════════{color_reset}{style_reset}");
    println!("{style_bold}{color_bright_cyan}  {VER}{color_reset}{style_reset}"); // VER statt hardcoded!
    println!("{style_bold}{color_yellow}  ⚠️  DEVELOPMENT RELEASE ⚠️{color_reset}{style_reset}");
    println!("{style_bold}{color_bright_cyan}═══════════════════════════════════════{color_reset}{style_reset}");
    println!();
    let _ = login(); // "login"
    shell_loop()    // shell
}
