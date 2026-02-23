# mina-tx

`mina-tx` contains Mina transaction domain logic used by this workspace.

## What it provides

- Mina transaction types (`TransactionEnvelope`, legacy tx, zkApp tx)
- JSON serialization/deserialization for transaction payloads
- GraphQL request builders for broadcasting signed transactions
- Signature output types (`Sig`, `PubKeySer`, `TransactionSignature`)
- Base58 helpers used by Mina transaction/signature formatting

## BluePallas compatibility

BluePallas/FROST-specific bridge code lives in `bluepallas_compat`.
This includes conversions such as:

- `TransactionEnvelope -> PallasMessage`
- FROST key/signature conversions needed to build `TransactionSignature`

Core transaction modules stay focused on transaction modeling and serialization.
