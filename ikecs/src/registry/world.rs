pub trait Component {}

pub trait Entity {
    fn get_index(&self) -> u32;
    fn get_version(&self) -> u32;
}
