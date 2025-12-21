use alloc::vec::Vec;
use mina_hasher::Fp;

pub mod emptiable;
pub mod packable;
pub mod packed_input;

#[derive(PartialEq, Debug)]
pub enum BitData {
    U32 { val: u32 },
    U64 { val: u64 },
    BOOL { val: bool },
    BYTES { val: Vec<u8> },
}

pub trait Packable {
    fn pack(&self) -> PackedInput;
}

// Represents a random oracle input (ROInput) from mina-hasher but with a different structure
// that is easier to debug and work with
#[derive(Default)]
pub struct PackedInput {
    pub bits: Vec<BitData>,
    pub fields: Vec<Fp>,
}
