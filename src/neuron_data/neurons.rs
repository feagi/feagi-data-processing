#[derive(Clone, Debug, PartialEq)]
pub struct NeuronXYZP {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub p: f32
}

impl NeuronXYZP {
    pub fn new(x: u32, y: u32, z: u32, p: f32) -> Self {
        NeuronXYZP { x, y, z, p }
    }
    
    pub fn as_tuple(&self) -> (u32, u32, u32, f32) {
        (self.x, self.y, self.z, self.p)
    }
}