//! Low-level poseidon hash utilities for ZkApp transactions
use ark_ec::AdditiveGroup;
use ark_ff::BigInt;
use ark_ff::Field;
use mina_hasher::Fp;

use crate::{errors::BluePallasError, transactions::zkapp_tx::zkapp_packable::ROInput};

fn param_to_field_impl(param: &str, default: &[u8; 32]) -> Result<Fp, BluePallasError> {
    let param_bytes = param.as_bytes();
    let len = param_bytes.len();

    let mut fp = *default;
    fp[..len].copy_from_slice(param_bytes);

    Fp::from_random_bytes(&fp).ok_or_else(|| {
        BluePallasError::InvalidZkAppCommand("Failed to convert parameter to field".to_string())
    })
}

pub fn param_to_field(param: &str) -> Result<Fp, BluePallasError> {
    const DEFAULT: &[u8; 32] = b"********************\0\0\0\0\0\0\0\0\0\0\0\0";

    if param.len() > DEFAULT.len() {
        return Err(BluePallasError::InvalidZkAppCommand(
            "must be 20 byte maximum".to_string(),
        ));
    }

    param_to_field_impl(param, DEFAULT)
}

pub(crate) fn pack_to_fields(roi: ROInput) -> ROInput {
    let fields = roi.fields;
    let bits = roi.bits;

    if bits.is_empty() {
        return ROInput { bits, fields };
    }

    let mut packed_bits = Vec::new();
    let mut current_packed_field = Fp::ZERO;
    let mut current_size = 0;
    for bit_data in bits {
        let size = bit_data.bit_data_size();
        let field = bit_data.to_field();

        current_size += size;
        if current_size < 255 {
            current_packed_field =
                current_packed_field * Fp::from(BigInt::from(1u64) << size as u32) + field;
        } else {
            packed_bits.push(current_packed_field);
            current_size = size;
            current_packed_field = field;
        }
    }
    packed_bits.push(current_packed_field);
    ROInput {
        bits: vec![],
        fields: [fields, packed_bits].concat(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_to_field() {
        let prefix = "MinaAcctUpdateNode";
        let field = param_to_field(prefix).unwrap();
        assert_eq!(
            field.to_string(),
            "240723076190006710499563866323038773312427551053"
        );
    }
}
