use crate::main;
use crate::VER;
use crate::DEV;
use crate::tools::misc::pause;

pub fn help() {
    println!("The following commands are implemented currently:");
    println!("help - display this command listing.");
    println!("info - display information about shell.");
    println!("todo - open the to-do list.");
    pause();
    help2();

    fn help2() {
        println!("exit - exit the shell.");
        println!("touch - create a textfile.");
        println!("echo - append content to a textfile.");
        pause();
        help3();
    }

    fn help3() {
        println!("cat - read from a file.");
        println!("path - gets the current directory.");
        println!("mkdir - create a directory.");
        pause();
        help4();
    }

    fn help4() {
        println!("rmdir - remove a directory.");
        println!("rnd - rename a directory.");
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
    println!("Improve working with files and paths.");
    println!("Improve the help command.");
    main();
}