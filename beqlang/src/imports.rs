use std::collections::HashSet;
use std::collections::HashMap;

pub struct Imports{
    lib_inclusions: HashSet<String>,
    file_inclusions: HashMap<String, HashSet<String>>,
}

impl Imports{
    pub fn new() -> Self{
        let lib_inclusions: HashSet<String> = HashSet::new();
        let file_inclusions: HashMap<String, HashSet<String>> = HashMap::new();

        return Self{
            lib_inclusions,
            file_inclusions,
        }
    }
    pub fn add_file(&mut self, file_name: String){
        let file_inclusion: HashSet<String> = HashSet::new();
        self.file_inclusions.insert(file_name, file_inclusion);
    }
    pub fn add_import(&mut self, file_name: String, import: String){
        self.lib_inclusions.insert(import.clone());
        if !self.file_inclusions.contains_key(&file_name){
            self.add_file(file_name.clone());
        }
        self.file_inclusions.get_mut(&file_name).unwrap().insert(import);
    }
    pub fn add_imports(&mut self, file_name: String, imports: Vec<String>){
        for import in imports{
            self.add_import(file_name.clone(), import);
        }
    }
    pub fn get_import_set(&self, file_name: String) -> HashSet<String>{
        return self.file_inclusions.get(&file_name).unwrap().clone();
    }
}
