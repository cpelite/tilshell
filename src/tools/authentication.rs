use std::{io};
use password_auth::{generate_hash, verify_password};
use std::fs::OpenOptions;
use std::io::Write;

pub fn setup_pw() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter password to hash:");
    let mut password = String::new();
    io::stdin().read_line(&mut password)?;

    let hash = generate_hash(password.trim());

    let mut file = OpenOptions::new()
    .append(true)
        .open(".env")?;

    writeln!(file, "\nSHELL_HASH_R=\"{}\"", hash)?;
    println!("Hash written to .env!");
    Ok(())
}

pub fn verify_pw(input: &str) -> bool {
    let content = std::fs::read_to_string(".env").unwrap_or_default();

    let stored_hash = content
        .lines()
        .find(|l| l.starts_with("SHELL_HASH_R="))
        .map(|l| l.trim_start_matches("SHELL_HASH_R=").trim_matches('"').to_string());

    match stored_hash {
        Some(hash) => match verify_password(input, &hash) {
            Ok(_) => true,
            Err(_) => false,
        },
        None => {
            println!("No password set up yet!");
            false
        }
    }
}