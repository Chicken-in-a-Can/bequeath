use std::collections::HashSet;
use crate::build::build::BuildType;


const DEFAULT_DIR_CREATION: BuildType = BuildType::FRESH;
const ALTERNATE_DIR_CREATION: BuildType = BuildType::UPDATE;

const DEFAULT_COMPILE_TYPE: BuildType = BuildType::DEBUG;
const ALTERNATE_COMPILE_TYPE: BuildType = BuildType::RELEASE;

pub fn gen_build_dir(build_options: HashSet<BuildType>){
    let dir_creation: BuildType = if build_options.contains(&DEFAULT_DIR_CREATION){
        DEFAULT_DIR_CREATION
    } else{
        ALTERNATE_DIR_CREATION
    };
    let compile_type: BuildType = if build_options.contains(&DEFAULT_COMPILE_TYPE){
        DEFAULT_COMPILE_TYPE
    } else {
        ALTERNATE_COMPILE_TYPE
    };
}
