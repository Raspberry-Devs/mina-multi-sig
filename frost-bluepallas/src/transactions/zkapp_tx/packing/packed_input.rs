use crate::transactions::zkapp_tx::packing::{BitData, PackedInput};
use alloc::vec::Vec;
use ark_ff::{AdditiveGroup, Field};
use mina_hasher::{Fp, ROInput};

impl BitData {
    pub fn bit_data_size(&self) -> usize {
        match self {
            BitData::U32 { .. } => 32,
            BitData::U64 { .. } => 64,
            BitData::BOOL { .. } => 1,
            BitData::BYTES { val } => val.len() * 8,
        }
    }

    pub fn to_field(&self) -> Fp {
        match self {
            BitData::U32 { val } => Fp::from(*val as u64),
            BitData::U64 { val } => Fp::from(*val),
            BitData::BOOL { val } => {
                if *val {
                    Fp::ONE
                } else {
                    Fp::ZERO
                }
            }
            BitData::BYTES { val } => {
                let mut bytes = [0u8; 32];
                let len = val.len().min(32);
                bytes[..len].copy_from_slice(&val[..len]);
                Fp::from_random_bytes(&bytes).expect("Failed to convert bytes to field")
            }
        }
    }
}

// Represents bits as tuples simillarly as o1js in Typescript
impl PackedInput {
    /// Create a new empty random oracle input
    pub fn new() -> Self {
        PackedInput {
            fields: vec![],
            bits: Vec::new(),
        }
    }

    pub fn append_packedinput(mut self, mut roi: PackedInput) -> Self {
        self.fields.append(&mut roi.fields);
        self.bits.extend(roi.bits);
        self
    }

    pub fn append_field(mut self, f: Fp) -> Self {
        self.fields.push(f);
        self
    }

    pub fn append_bool(mut self, b: bool) -> Self {
        self.bits.push(BitData::BOOL { val: b });
        self
    }

    pub fn append_u32(mut self, x: u32) -> Self {
        self.bits.push(BitData::U32 { val: x });
        self
    }

    pub fn append_u64(mut self, x: u64) -> Self {
        self.bits.push(BitData::U64 { val: x });
        self
    }

    pub fn append_bytes(mut self, bytes: &[u8]) -> Self {
        self.bits.push(BitData::BYTES {
            val: bytes.to_vec(),
        });
        self
    }

    pub fn to_mina_hasher_roi(self) -> ROInput {
        let mut inputs = ROInput::new();

        for field in self.fields {
            inputs = inputs.append_field(field)
        }

        for bit_data in self.bits {
            match bit_data {
                BitData::U32 { val } => {
                    inputs = inputs.append_u32(val);
                }
                BitData::U64 { val } => {
                    inputs = inputs.append_u64(val);
                }
                BitData::BOOL { val } => {
                    inputs = inputs.append_bool(val);
                }
                BitData::BYTES { val } => {
                    inputs = inputs.append_bytes(val.as_slice());
                }
            }
        }

        inputs
    }
}
