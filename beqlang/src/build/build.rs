use std::{
    env::set_current_dir,
    collections::HashSet,
    process::{
        Output,
        Command
    }
};

use crate::build::build_dir;

pub const DEFAULT_DIR_CREATION: BuildType = BuildType::Fresh;
pub const ALTERNATE_DIR_CREATION: BuildType = BuildType::Update;

pub const DEFAULT_COMPILE_TYPE: BuildType = BuildType::Debug;
pub const ALTERNATE_COMPILE_TYPE: BuildType = BuildType::Release;

pub const DEFAULT_RUN_TYPE: BuildType = BuildType::RunRelease;
pub const ALTERNATE_RUN_TYPE: BuildType = BuildType::RunDebug;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum BuildType {
    Fresh,
    Update,
    Debug,
    Release,
    RunDebug,
    RunRelease,
}

pub struct Build{
    build_options: HashSet<BuildType>,
    build_dir: String,
}

impl Build{
    pub fn new(build_options: HashSet<BuildType>, build_dir: String) -> Self{
        return Self{
            build_options,
            build_dir,
        }
    }
    pub fn build(&self){
        let build_dir_name = build_dir::gen_build_dir(self.build_options.clone(), self.build_dir.clone());
        match set_current_dir(build_dir_name){
            Ok(_result) => (),
            Err(_e) => panic!("Error\nCould not change directory"),
        }
        let build_command: String;
        if self.build_options.contains(&BuildType::Debug) || (!self.build_options.contains(&BuildType::Release) && DEFAULT_COMPILE_TYPE == BuildType::Debug){
            build_command = String::from("cargo build");
        } else{
            build_command = String::from("cargo build --release");
        }
        let rust_init: Output = if cfg!(target_os = "windows"){
            Command::new("cmd")
                .args(["/C", &build_command])
                .output()
                .expect("Failed to build cargo project")
        } else{
            Command::new("sh")
                .args(["-c", &build_command])
                .output()
                .expect("Failed to build cargo project")
        };
        let _init_output = rust_init.stdout;
    }
}
