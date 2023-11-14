use std::{env::set_current_dir, collections::HashSet};

use crate::build::build_dir;

#[derive(Eq, PartialEq, Hash)]
pub enum BuildType {
    Fresh,
    Update,
    Debug,
    Release,
    RunDebug,
    RunRelease,
}

pub struct Build{}

impl Build{
    pub fn build(build_options: HashSet<BuildType>, build_dir: String){
        let build_dir_name = build_dir::gen_build_dir(build_options, build_dir);
        let _result = set_current_dir(build_dir_name);
    }
}
