#![allow(dead_code)]

// Concrete data types used by grap-rs generics in this binary crate.
// All types are no_std friendly.

/// Unit data: contains a single u32 value.
#[derive(Clone, Copy, Debug, Default)]
pub struct UnitData {
    value: u32,
}

/// Edge data: contains a u32 field that should mirror the edge id (index).
#[derive(Clone, Copy, Debug, Default)]
pub struct EdgeData {
    id_value: u32,
}

// Traits for simple set/modify/read operations

/// A trait to get/set/update a single u32 value.
pub trait U32Value {
    fn value(&self) -> u32;
    fn set_value(&mut self, v: u32);
    /// Update the value using a provided function.
    fn update_value<F: FnOnce(u32) -> u32>(&mut self, f: F) {
        let cur = self.value();
        self.set_value(f(cur));
    }
}

impl UnitData {
    #[inline]
    pub const fn new(value: u32) -> Self { Self { value } }
}

impl U32Value for UnitData {
    #[inline]
    fn value(&self) -> u32 { self.value }
    #[inline]
    fn set_value(&mut self, v: u32) { self.value = v; }
}

/// A trait specific to EdgeData for get/set/update the id-like field.
pub trait EdgeIdLike {
    fn id_value(&self) -> u32;
    fn set_id_value(&mut self, v: u32);
    fn update_id_value<F: FnOnce(u32) -> u32>(&mut self, f: F) {
        let cur = self.id_value();
        self.set_id_value(f(cur));
    }
}

impl EdgeData {
    #[inline]
    pub const fn new_with_id(id: u32) -> Self { Self { id_value: id } }
    #[inline]
    pub const fn new() -> Self { Self { id_value: 0 } }
}

impl EdgeIdLike for EdgeData {
    #[inline]
    fn id_value(&self) -> u32 { self.id_value }
    #[inline]
    fn set_id_value(&mut self, v: u32) { self.id_value = v; }
}
