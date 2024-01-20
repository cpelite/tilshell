use crate::main;
use crate::VER;
use crate::DEV;
use crate::tools::misc::pause;

pub fn help() {
    println!("The following commands are implemented currently:");
    println!("help - displays this command listing.");
    println!("info - displays information about shell.");
    println!("todo - opens the to-do list.");
    pause();
    help2();

    fn help2() {
        println!("exit - exits the shell.");
        println!("touch - creates a textfile.");
        println!("echo - appends content to a textfile.");
        pause();
        help3();
    }

    fn help3() {
        println!("cat - reads from a file.");
        println!("path - gets the current directory.");
        pause();
        main();
    }
}

pub fn info() {
    println!("{VER}");
    println!("{DEV}");
    main();
}

pub fn exit() {
    std::process::exit(0);
}

pub fn todo() {
    println!("ToDo:");
    println!("Find a way to remove line breaks..");
    println!("Add a calculator.");
    println!("Adding what ever comes to my mind.");
    main();
}
