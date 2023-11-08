use std::fs;
use crate::constants;

pub struct InputFile{
    file_contents: Vec<String>,
}

impl InputFile{
    pub fn new(file_name: &str) -> Self{
        let contents_result = fs::read_to_string(file_name);
        let contents = match contents_result{
            Ok(contents) => contents,
            Err(e) => panic!("Error: {e}\n\nExiting"),
        };
        let mut file_contents: Vec<String> = contents
            .split("\n")
            .map(|line| line
                .to_string()
                .trim()
                .to_owned()
            ).collect();
        let mut line_number: isize = 0;
        while (line_number as usize) < file_contents.len(){
            if file_contents[line_number as usize] == ""{
                file_contents.remove(line_number as usize);
                line_number = line_number - 1;
            }
            line_number += 1;
        }

        return Self{
            file_contents,
        };
    }
    pub fn get_imports(&self) -> Vec<String>{
        let mut import_vec: Vec<String> = Vec::new();
        for line in self.file_contents.clone(){
            if line.len() >= 4 && &line[0..4] == "use "{
                if constants::DEBUG{
                    println!("{}", line[4..].to_owned());
                }
                import_vec.push(line[4..].to_owned())
            }
        }
        return import_vec;
    }
}
