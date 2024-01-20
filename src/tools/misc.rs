use std::env;
use crate::main;

pub fn pause() {
    let mut _pause = String::new();

    std::io::stdin()
        .read_line(&mut _pause)
        .expect("Failed to read line.");
}

pub fn currdir() {
    let dir = env::current_dir().unwrap();
    let dirprint = dir.display();
    println!("{dirprint}");
    main();
}