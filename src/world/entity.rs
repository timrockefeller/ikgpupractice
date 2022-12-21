use ikecs::{Component, Entity};


struct TransformComponent {}

impl Component for TransformComponent {}

struct Uobject {
    idx: u32,
    ver: u32,
}

impl Entity for Uobject {
    fn get_index(&self) -> u32 {
        self.idx
    }
    fn get_version(&self) -> u32 {
        self.ver
    }
}
