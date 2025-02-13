use std::env;
use crate::main;

pub fn currdir() {
    let dir = env::current_dir().unwrap();
    let dirprint = dir.display();
    println!("{dirprint}");
    main();
}