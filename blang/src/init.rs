use std::fs;

pub fn read_directory(input_directory: &str, file_paths: &mut Vec<String>){
    let entries_result = fs::read_dir(input_directory);
    let entries = match entries_result{
        Ok(paths) => paths,
        Err(e) => panic!("Error: {e}\n\nExiting!"),
    };
    for entry in entries{
        let entry = match entry{
            Ok(entry) => entry,
            Err(e) => panic!("Error, {e}\n\nExiting!"),
        };
        let metadata = match entry.metadata(){
            Ok(metadata) => metadata,
            Err(e) => panic!("Error, {e}\n\nExiting!"),
        };
        if metadata.is_dir(){
            let path = entry.path();
            let next_dir: &str = match path.to_str(){
                Some(next_dir) => next_dir,
                None => "",
            };
            read_directory(next_dir, file_paths);
        } else if metadata.is_file(){
            let path = entry.path();
            match path.to_str(){
                Some(filename) => file_paths.push(filename.to_owned()),
                None => (),
            }
        }
    }
}

pub fn get_filetype(file_name: String) -> String{
    let filetype: String;

    filetype = match file_name.rsplit_once("."){
        Some(split_file) => split_file.1.to_owned(),
        None => "".to_owned(),
    };

    return filetype;
}
