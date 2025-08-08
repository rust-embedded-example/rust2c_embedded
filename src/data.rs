#![allow(dead_code)]

// Concrete data types used by grap-rs generics in this binary crate.
// All types are no_std friendly.

/// Unit data as a C-compatible structure.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct UnitData {
    pub value: u32,
}

/// Edge data as a C-compatible structure.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeData {
    pub id_value: u32,
}

/// Node data as a C-compatible structure.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct NodeData {
    pub id: usize,
}

// Inherent methods on UnitData (no trait)
impl UnitData {
    #[inline]
    pub const fn new(value: u32) -> Self { Self { value } }
    #[inline]
    pub fn value(&self) -> u32 { self.value }
    #[inline]
    pub fn set_value(&mut self, v: u32) { self.value = v; }
}



impl EdgeData {
    #[inline]
    pub const fn new_with_id(id: u32) -> Self { Self { id_value: id } }
    #[inline]
    pub const fn new() -> Self { Self { id_value: 0 } }
}


