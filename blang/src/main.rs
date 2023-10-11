use blang::init::Init;

use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let _init = Init::new(&args[1]);
}
