#[derive(Eq, PartialEq, Hash)]
pub enum BuildType {
    FRESH,
    UPDATE,
    DEBUG,
    RELEASE,
}
