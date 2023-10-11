use std::fs;

pub struct Init{
    input_files: Vec<String>,
}

impl Init{
    pub fn new(input_directory: &str) -> Self{
        let paths_result = fs::read_dir(input_directory);
        let paths = match paths_result{
            Ok(paths) => paths,
            Err(e) => panic!("Error: {e}\n\n Exiting!"),
        };
        let input_files: Vec<String> = paths.map(
            |path| path
                .unwrap()
                .path()
                .to_str()
                .unwrap()
                .to_owned()
            ).collect();
        return Self{
            input_files,
        };
    }
}
