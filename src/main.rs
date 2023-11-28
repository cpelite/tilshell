use crate::tools::initandmain;
use crate::tools::fileops;

mod tools {
    pub mod initandmain;
    pub mod fileops;
}

fn main() {
    initandmain::init();
}