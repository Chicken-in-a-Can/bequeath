use std::fs;

pub fn read_directory(input_directory: &str) -> Vec<String>{
    let paths_result = fs::read_dir(input_directory);
    let paths = match paths_result{
        Ok(paths) => paths,
        Err(e) => panic!("Error: {e}\n\nExiting!"),
    };
    let input_files: Vec<String> = paths.map(
        |path| path
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .to_owned()
        ).collect();
    return input_files;
}

pub fn get_filetype(file_name: String) -> String{
    let filetype: String;

    filetype = match file_name.rsplit_once("."){
        Some(split_file) => split_file.1.to_owned(),
        None => "".to_owned(),
    };

    return filetype;
}
