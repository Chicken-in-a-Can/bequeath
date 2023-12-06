use crate::file::infile::InputFile;
use crate::constants;

pub struct File{
    file_contents: Vec<String>
}

impl File{
    pub fn new(input_file: InputFile) -> Self{
        return Self{
            file_contents: input_file.get_contents();
        }
    }

    pub fn get_imports(&mut self) -> Vec<String>{
        let mut import_vec: Vec<String> = Vec::new();
        let mut index: isize = 0;
        for line in self.file_contents.clone(){
            if line.len() >= 4 && &line[0..4] == "use "{
                if constants::DEBUG{
                    println!("{}", line[4..].to_owned());
                }
                import_vec.push(line[4..].to_owned());
                self.file_contents.remove(index as usize);
                index -= 1;
            }
            index += 1;
        }
        return import_vec;
    }
}
