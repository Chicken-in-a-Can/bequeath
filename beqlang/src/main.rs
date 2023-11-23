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
    for file_name in read_dir{
        let filetype = init::get_filetype(file_name.clone());
        if constants::FILETYPES.contains(&filetype.as_str()){
            let mut infile: InputFile = InputFile::new(&file_name);
            imports.add_imports(file_name.clone(), infile.get_imports());
            if constants::DEBUG{
                println!("{}", file_name);
            }
        }
    }
}
