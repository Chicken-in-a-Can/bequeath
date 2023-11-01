use blang::init;
use blang::constants;
use blang::file::infile::InputFile;

use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut read_dir: Vec<String> = Vec::new();
    init::read_directory(&args[1], &mut read_dir);
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
