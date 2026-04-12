use std::{env, io};
use std::io::Write;
use crate::parse_command;
use crate::tools::parser::{parse_command_bene, parse_command_guest};
use dotenv::dotenv;

//rudimentary login system, based on dotenv
pub fn login() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let valid_user = env::var("SHELL_USER").unwrap_or_else(|_| "lynnf".to_string());
    loop {
        println!("Please enter your username");

        let mut username = String::new();
        io::stdin().read_line(&mut username)?;

        match username.trim() {
            u if u == valid_user => {
                println!("Welcome, {}.", u);
                break;
            }

            "guest" => {
                println!("Welcome, Guest. Sub-Shell for guests is active.");
                let _ = guest_shell();
                break;
            }

            "bene" => {
                println!("Welcome, Bene. Shell-instance is active.");
                let _ = bene_shell();
                break;
            }

            _ => {
                println!("Authentication failure!");
                println!("Restarting authentication!");
            }
        }
    }
    Ok(())
}

/* old authentication system, which can be defined as a very early prototype,
        but is more likely just flaming garbage..


        if username.trim() == "lynnf" {
            println!("Welcome, Lynn.");
            break;
        } else if username.trim() == "guest" {
            println!("Welcome, Guest. Sub-shell for guests is active.");
            let _ = guest_shell();
            break;
        } else {
            println!("Authentication failure!");
            println!("Restarting authentication!");
        }
    }
    Ok(())
} */

// looping after login - "root" shell
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

// looping after login - guest shell
pub fn guest_shell() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let curr_path = env::current_dir()?;
        print!("{} ", curr_path.display());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        parse_command_guest(&input);
    }
}

// looping after login - bene shell
pub fn bene_shell() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let curr_path = env::current_dir()?;
        print!("{} ", curr_path.display());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        parse_command_bene(&input);
    }
}