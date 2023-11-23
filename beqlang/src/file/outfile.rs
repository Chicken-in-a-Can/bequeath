use std::{fs::File, io::Write};
use crate::imports::Imports;

pub struct OutputFile{
    file_contents: Vec<String>,
    file_name: String,
}

impl OutputFile{
    pub fn new(file_name: String) -> Self{
        let file_contents: Vec<String> = Vec::new();
        return Self{
            file_contents,
            file_name
        };
    }
    pub fn write(&self){
        let file_open_result = File::create(&self.file_name);
        let mut file_to_write = match file_open_result{
            Ok(file) => file,
            Err(e) => panic!("Error: {e}\n\nExiting"),
        };
        for line in self.file_contents.clone(){
            let _result = file_to_write.write_all(String::from(line + "\n").as_bytes());
        }
    }
    pub fn add_imports(&mut self, imports: Imports){
        let imports = imports.get_import_set(self.file_name.clone());
        for import in imports{
            self.file_contents.insert(0, format!("use {};", import));
        }
    }
}
