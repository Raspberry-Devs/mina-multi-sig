#![allow(dead_code)]
use mina_hasher::{Hashable, ROInput};
use mina_signer::{CompressedPubKey, NetworkId, PubKey};

/// Copied from https://github.com/o1-labs/proof-systems/blob/master/signer/tests/transaction.rs
const MEMO_BYTES: usize = 34;
const TAG_BITS: usize = 3;
const PAYMENT_TX_TAG: [bool; TAG_BITS] = [false, false, false];
const DELEGATION_TX_TAG: [bool; TAG_BITS] = [false, false, true];

#[derive(Clone)]
pub struct Transaction {
    // Common
    pub fee: u64,
    pub fee_token: u64,
    pub fee_payer_pk: CompressedPubKey,
    pub nonce: u32,
    pub valid_until: u32,
    pub memo: [u8; MEMO_BYTES],
    // Body
    pub tag: [bool; TAG_BITS],
    pub source_pk: CompressedPubKey,
    pub receiver_pk: CompressedPubKey,
    pub token_id: u64,
    pub amount: u64,
    pub token_locked: bool,
}

impl Hashable for Transaction {
    type D = NetworkId;

    fn to_roinput(&self) -> ROInput {
        let mut roi = ROInput::new()
            .append_field(self.fee_payer_pk.x)
            .append_field(self.source_pk.x)
            .append_field(self.receiver_pk.x)
            .append_u64(self.fee)
            .append_u64(self.fee_token)
            .append_bool(self.fee_payer_pk.is_odd)
            .append_u32(self.nonce)
            .append_u32(self.valid_until)
            .append_bytes(&self.memo);

        for tag_bit in self.tag {
            roi = roi.append_bool(tag_bit);
        }

        roi.append_bool(self.source_pk.is_odd)
            .append_bool(self.receiver_pk.is_odd)
            .append_u64(self.token_id)
            .append_u64(self.amount)
            .append_bool(self.token_locked)
    }

    fn domain_string(network_id: NetworkId) -> Option<String> {
        // Domain strings must have length <= 20
        match network_id {
            NetworkId::MAINNET => "MinaSignatureMainnet",
            NetworkId::TESTNET => "CodaSignature",
        }
        .to_string()
        .into()
    }
}

impl Transaction {
    pub fn new_payment(from: PubKey, to: PubKey, amount: u64, fee: u64, nonce: u32) -> Self {
        Transaction {
            fee,
            fee_token: 1,
            fee_payer_pk: from.into_compressed(),
            nonce,
            valid_until: u32::MAX,
            memo: core::array::from_fn(|i| (i == 0) as u8),
            tag: PAYMENT_TX_TAG,
            source_pk: from.into_compressed(),
            receiver_pk: to.into_compressed(),
            token_id: 1,
            amount,
            token_locked: false,
        }
    }

    pub fn new_delegation(from: PubKey, to: PubKey, fee: u64, nonce: u32) -> Self {
        Transaction {
            fee,
            fee_token: 1,
            fee_payer_pk: from.into_compressed(),
            nonce,
            valid_until: u32::MAX,
            memo: core::array::from_fn(|i| (i == 0) as u8),
            tag: DELEGATION_TX_TAG,
            source_pk: from.into_compressed(),
            receiver_pk: to.into_compressed(),
            token_id: 1,
            amount: 0,
            token_locked: false,
        }
    }

    pub fn set_valid_until(mut self, global_slot: u32) -> Self {
        self.valid_until = global_slot;

        self
    }

    pub fn set_memo(mut self, memo: [u8; MEMO_BYTES - 2]) -> Self {
        self.memo[0] = 0x01;
        self.memo[1] = (MEMO_BYTES - 2) as u8;
        self.memo[2..].copy_from_slice(&memo[..]);

        self
    }

    pub fn set_memo_str(mut self, memo: &str) -> Self {
        self.memo[0] = 0x01;
        self.memo[1] = core::cmp::min(memo.len(), MEMO_BYTES - 2) as u8;
        let memo = format!("{memo:\0<32}"); // Pad user-supplied memo with zeros
        self.memo[2..]
            .copy_from_slice(&memo.as_bytes()[..core::cmp::min(memo.len(), MEMO_BYTES - 2)]);
        // Anything beyond MEMO_BYTES is truncated

        self
    }
}
