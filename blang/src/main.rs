use blang::init;
use blang::constants;
use blang::file::infile::InputFile;

use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let read_dir = init::read_directory(&args[1]);
    for file in read_dir{
        let filetype = init::get_filetype(file.clone());
        if filetype == "rs"{
            let infile: InputFile = InputFile::new(&file);
            infile.get_imports();
            if constants::DEBUG{
                println!("{}", file);
            }
        }
    }
}
