use std::collections::HashSet;
use crate::build::build::{self, BuildType};

use std::fs;
use std::process::{Output, Command};

pub fn gen_build_dir(build_options: HashSet<BuildType>, dir_name: String) -> String{
    let dir_creation: BuildType = if build_options.contains(&build::DEFAULT_DIR_CREATION){
        build::DEFAULT_DIR_CREATION
    } else{
        build::ALTERNATE_DIR_CREATION
    };

    let _compile_type: BuildType = if build_options.contains(&build::DEFAULT_COMPILE_TYPE){
        build::DEFAULT_COMPILE_TYPE
    } else {
        build::ALTERNATE_COMPILE_TYPE
    };

    let run_type: BuildType = if build_options.contains(&build::DEFAULT_RUN_TYPE){
        build::DEFAULT_RUN_TYPE
    } else {
        build::ALTERNATE_RUN_TYPE
    };


    let build_dir_name: String = String::from(dir_name + "_build" );

    if run_type == BuildType::Update && !fs::metadata(build_dir_name.clone()).is_ok(){
        panic!("Error\nUpdate build type cannot be used if directory doesn't exist");
    }

    if dir_creation == BuildType::Fresh {
        let _result = fs::create_dir(format!("./{}", build_dir_name));
        let rust_init: Output = if cfg!(target_os = "windows"){
            Command::new("cmd")
                .args(["/C", "cargo init {build_dir_name}"])
                .output()
                .expect("Failed to initialize cargo in directory")
        } else{
            Command::new("sh")
                .args(["-c", "cargo init {build_dir_name}"])
                .output()
                .expect("Failed to initialize cargo in directory")
        };
        let _init_output = rust_init.stdout;
    }

    return build_dir_name;
}
