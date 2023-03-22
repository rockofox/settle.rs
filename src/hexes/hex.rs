use std::sync::Arc;

use crate::resources::resource::Resource;

pub type HexPosition = (u64, u64);

#[derive(Debug)]
pub struct HexCoords {
    pos: HexPosition,
}

impl HexCoords {
    fn getPos(&self) -> HexPosition {
        self.pos
    }

    fn setPos(&mut self, coords: HexPosition) {
        self.pos = coords;
    }
}

pub struct DiceValue {
    pub value: u64,
}

impl PartialEq<u64> for DiceValue {
    fn eq(&self, other: &u64) -> bool {
        self.value == *other
    }
}

pub fn create_hex(
    pos: (u64, u64),
    value: u64,
    resource: Arc<Resource>,
) -> (HexCoords, DiceValue, Arc<Resource>) {
    (HexCoords { pos }, DiceValue { value }, resource)
}
