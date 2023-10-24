pub struct OutputFile{
    file_contents: Vec<String>,
}

impl OutputFile{
    pub fn new() -> Self{
        let file_contents: Vec<String> = Vec::new();
        return Self{
            file_contents,
        };
    }
}
