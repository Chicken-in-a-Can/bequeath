#[derive(Eq, PartialEq, Hash)]
pub enum BuildType {
    Fresh,
    Update,
    Debug,
    Release,
    RunDebug,
    RunRelease,
}
