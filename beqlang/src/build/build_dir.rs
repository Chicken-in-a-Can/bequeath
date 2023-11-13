use std::collections::HashSet;
use crate::build::build::BuildType;

use std::fs;
use std::process::{Output, Command};


const DEFAULT_DIR_CREATION: BuildType = BuildType::Fresh;
const ALTERNATE_DIR_CREATION: BuildType = BuildType::Update;

const DEFAULT_COMPILE_TYPE: BuildType = BuildType::Debug;
const ALTERNATE_COMPILE_TYPE: BuildType = BuildType::Release;

const DEFAULT_RUN_TYPE: BuildType = BuildType::RunRelease;
const ALTERNATE_RUN_TYPE: BuildType = BuildType::RunDebug;

pub fn gen_build_dir(build_options: HashSet<BuildType>, dir_name: String){
    let dir_creation: BuildType = if build_options.contains(&DEFAULT_DIR_CREATION){
        DEFAULT_DIR_CREATION
    } else{
        ALTERNATE_DIR_CREATION
    };

    let _compile_type: BuildType = if build_options.contains(&DEFAULT_COMPILE_TYPE){
        DEFAULT_COMPILE_TYPE
    } else {
        ALTERNATE_COMPILE_TYPE
    };

    let _run_type: BuildType = if build_options.contains(&DEFAULT_RUN_TYPE){
        DEFAULT_RUN_TYPE
    } else {
        ALTERNATE_RUN_TYPE
    };


    let build_dir_name: String = String::from(dir_name + "_build" );

    if dir_creation == BuildType::Fresh {
        let _result = fs::create_dir(format!("./{}", build_dir_name));
        let rust_init: Output = if cfg!(target_os = "windows"){
            Command::new("cmd")
                .args(["/C", "cargo init"])
                .output()
                .expect("Failed to initialize cargo in directory")
        } else{
            Command::new("sh")
                .args(["-c", "cargo init"])
                .output()
                .expect("Failed to initialize cargo in directory")
        };

        let _init_output = rust_init.stdout;
    }
}
