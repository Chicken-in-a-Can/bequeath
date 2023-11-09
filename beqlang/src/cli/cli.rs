use std::collections::HashSet;
use std::process::exit;

use crate::build::build::BuildType;
use crate::constants;

pub struct CLI{
    args: Vec<String>,
    build_args: HashSet<BuildType>,
}

impl CLI{
    pub fn parse(args: Vec<String>) -> Self{
        let mut cli = CLI::new(args);

        cli.parse_args();

        return cli;
    }
    pub fn new(args: Vec<String>) -> Self{
        let build_args: HashSet<BuildType> = HashSet::new();
        return Self{
            args,
            build_args,
        };
    }
    pub fn get_root_dir(&self) -> String{
        return self.args[self.args.len() - 1].clone();
    }
    fn parse_args(&mut self){
        for arg in self.args.clone(){
            match arg.as_str(){
                "--fresh" => {
                    self.build_args.insert(BuildType::FRESH);
                },
                "--update" => {
                    self.build_args.insert(BuildType::UPDATE);
                },
                "--release" => {
                    self.build_args.insert(BuildType::RELEASE);
                },
                "--debug" => {
                    self.build_args.insert(BuildType::DEBUG);
                },
                "--help" | "-h" => {
                    println!("
Bequeath version {}
Usage:
bequeath [args] $SRC_DIR

Args:
--help / -h:            Prints this help message
--fresh / -f:           Build in fresh build dir
--update / -u:          Updates existing build directory
--release / -r:         Compiles code for release. Takes longer, but runs faster
--debug / -d:           Compiles code for debug. Much faster, but runs slower. for debugging
                    ", constants::VERSION);
                   exit(0);
                }
                _ => {},
            }
        }
    }
}
