pub mod generic_tx;
pub mod legacy_tx;
pub mod zkapp_tx;

const MEMO_BYTES: usize = 34;
const MEMO_HEADER_BYTES: usize = 2; // 0x01 + length byte
