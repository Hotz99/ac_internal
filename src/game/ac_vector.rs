// AssaultCube custom vector type that holds pointers to enemies
// which are also of type Player
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AcVector {
    // pointer to array of player pointers
    pub player_addrs: usize,
    pub capacity: u32,
    pub elements: u32,
}
