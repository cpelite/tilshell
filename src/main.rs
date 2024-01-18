use crate::tools::shellmain;

mod tools {
    pub mod shellmain;
    pub mod fileops;
}

fn main() {
    shellmain::shmain();
}