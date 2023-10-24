use std::fs;

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
        let mut line_number = 0;
        while line_number < file_contents.len(){
            if file_contents[line_number] == ""{
                file_contents.remove(line_number);
                line_number = line_number - 1;
            }
            line_number += 1;
        }

        return Self{
            file_contents,
        };
    }
    pub fn get_imports(&self){
        for line in self.file_contents{
            if line[0..4] = "use "{}
        }
    }
}
