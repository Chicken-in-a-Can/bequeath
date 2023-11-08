pub struct CLI{
    args: Vec<String>
}

impl CLI{
    pub fn new(args: Vec<String>) -> Self{
        return Self{
            args,
        };
    }
    pub fn get_root_dir(&self) -> String{
        return self.args[self.args.len() - 1].clone();
    }
}
