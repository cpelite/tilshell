use std::{env, io};
use std::io::Write;
use crate::parse_command;
use crate::tools::authentication::*;
use crate::tools::roles::*;
use dotenv::dotenv;

// login system, based on dotenv
pub fn login() -> Result<UserRole, Box<dyn std::error::Error>> {
    dotenv().ok();
    let valid_user = env::var("SHELL_USER_R").unwrap_or_else(|_| "lynn".to_string());
    let bene_user = env::var("SHELL_USER_B").unwrap_or_else(|_| "bene".to_string());

    loop {
        println!("Please enter your username");

        let mut username = String::new();
        io::stdin().read_line(&mut username)?;

        match username.trim() {
            u if u == valid_user => {
                println!("Enter password:");
                let mut password = String::new();
                io::stdin().read_line(&mut password)?;
                if verify_pw(password.trim()) {
                    println!("Welcome, {}.", u);
                    return Ok(UserRole::Lynn);
                } else {
                    println!("Wrong password!");
                    println!("Restarting authentication!")
                }
            }

            u if u == bene_user => {
                println!("Welcome, {}. Your shell is active.", u);
                return Ok(UserRole::Bene);
            }

            "guest" => {
                println!("Welcome, Guest. Sub-Shell for guests is active.");
                return Ok(UserRole::Guest);
            }
            _ => {
                println!("Authentication failure!");
                println!("Restarting authentication!");
            }
        }
    }
}


// loop after login
pub fn shell_loop(role: UserRole) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let curr_path = env::current_dir()?;
        print!("{} ", curr_path.display());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        parse_command(&input, &role);
    }
}