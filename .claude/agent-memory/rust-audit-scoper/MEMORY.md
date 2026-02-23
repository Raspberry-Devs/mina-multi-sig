# Audit Scoper Memory -- mina-multi-sig

## Project Architecture
- Workspace: frost-bluepallas, mina-tx, mina-frost-client
- frost-bluepallas is `#![no_std]` (uses alloc), ~1024 LOC src
- BluePallas<M> is generic over ChallengeMessage trait
- PallasMessage (in mina-tx) is the concrete ChallengeMessage impl
- ChallengeMessage impl and type conversions are in mina-tx, gated behind `frost-bluepallas-compat` feature

## Critical Invariants
- Y-coordinate evenness enforced in pre_commitment_sign/pre_commitment_aggregate hooks
- H2 intentionally unimplemented (panics) -- challenge computed via ChallengeMessage trait
- PallasScalarField serialization is little-endian (comment at lib.rs:95 incorrectly says big-endian)
- PallasGroup serialization uses 96-byte buffer but compressed serialize
- PallasMessage::challenge() has unwrap_or_else fallback to TESTNET/legacy on deserialization failure

## Key Risk Areas
- frost-core v3.0.0-rc.0 -- RC status, known bug #1015 (signature share verification)
- 4 integration tests ignored due to upstream bug
- PubKey::from_point_unsafe() used in translate_pk -- name suggests reduced validation
- One expect() in lib code: PallasScalarField::serialize at lib.rs:86

## Dependency Sources
- mina-* crates from o1-labs/proof-systems git tag 0.3.0 (not crates.io)
- ark-* ecosystem v0.5.0
- Rust toolchain 1.92.0
