use std::{env, io};
use std::io::Write;
use crate::parse_command;

//rudimentary login system
pub fn login() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Please enter your username");
        let mut username = String::new();
        io::stdin().read_line(&mut username)?;
        if username.trim() == "lynnf" {
            println!("Welcome, Lynn.");
            break;
        } else if username.trim() == "guest" {
            println!("Welcome, Guest. Sub-shell for guests is active.");
            guest_shell();
            break;
        } else {
            println!("Authentication failure!");
            println!("Restarting authentication!");
        }
    }
    Ok(())
}

// looping after login - normal shell
pub fn shell_loop() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let curr_path = env::current_dir()?;
        print!("{} ", curr_path.display());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        parse_command(&input);
    }
}

// guest shell
pub fn guest_shell() {
    println!("Well.. that's awkward - but the sub-shell for guests is still in work. Sorry!");
}