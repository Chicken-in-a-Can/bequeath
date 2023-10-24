use blang::init;

use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let _init = init::read_directory(&args[1]);
}
