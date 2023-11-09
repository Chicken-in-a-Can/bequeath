use beqlang::init;
use beqlang::constants;
use beqlang::file::infile::InputFile;
use beqlang::imports::Imports;
use beqlang::cli::cli::CLI;

use std::env;

fn main(){
    let mut imports = Imports::new();

    let args: Vec<String> = env::args().collect();
    let mut read_dir: Vec<String> = Vec::new();

    let cli: CLI = CLI::parse(args);

    init::read_directory(&cli.get_root_dir(), &mut read_dir);
    for file in read_dir{
        let filetype = init::get_filetype(file.clone());
        if filetype == "rs"{
            let infile: InputFile = InputFile::new(&file);
            imports.add_imports(file.clone(), infile.get_imports());
            if constants::DEBUG{
                println!("{}", file);
            }
        }
    }
}
