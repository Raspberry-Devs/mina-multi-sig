mod common;

#[cfg(not(feature = "mesa-hardfork"))]
mod pre_mesa;

#[cfg(feature = "mesa-hardfork")]
mod mesa;

pub use common::{
    decode_memo_from_base58, parse_expected_hash, HashWithPrefixTestVector, ZkAppTestVector,
};

#[cfg(not(feature = "mesa-hardfork"))]
pub use pre_mesa::{get_hash_with_prefix_test_vectors, get_zkapp_test_vectors};

#[cfg(feature = "mesa-hardfork")]
pub use mesa::{get_hash_with_prefix_test_vectors, get_zkapp_test_vectors};
