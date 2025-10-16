use ark_ff::Field;
use mina_hasher::{Fp, Hashable, ROInput};

use crate::errors::BluePallasError;

#[derive(Clone)]
pub struct HashableField(Fp);

impl Hashable for HashableField {
    type D = ();

    fn to_roinput(&self) -> ROInput {
        ROInput::new().append_field(self.0)
    }

    fn domain_string(_id: Self::D) -> Option<String> {
        None
    }
}

impl From<Fp> for HashableField {
    fn from(field: Fp) -> Self {
        HashableField(field)
    }
}

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

    if param.len() > 20 {
        return Err(BluePallasError::InvalidZkAppCommand(
            "must be 20 byte maximum".to_string(),
        ));
    }

    param_to_field_impl(param, DEFAULT)
}

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
