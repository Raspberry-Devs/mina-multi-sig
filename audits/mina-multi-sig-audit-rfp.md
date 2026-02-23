---
title: "Security Audit RFP -- frost-bluepallas Crate"
date: 2026-02-23
version: "2.0"
---

# Request for Proposal (RFP)
## Security Audit -- `frost-bluepallas` (and Security-Relevant Bridging Code)

---

### 1. Introduction

`mina-multi-sig` is a Rust project that implements **off-chain threshold multisig wallets for the Mina Protocol** using the **FROST (Flexible Round-Optimized Schnorr Threshold Signatures) scheme**. A group of signers collaboratively produce a single Mina-compatible Schnorr signature without placing multisig logic on-chain.

The repository is a Cargo workspace with three crates:

| Crate | Role |
|---|---|
| `frost-bluepallas` | Core cryptography library. Implements the FROST ciphersuite over Mina's Pallas curve with Poseidon hashing. Generic over a `ChallengeMessage` trait parameter. |
| `mina-tx` | Mina transaction types, serialisation, and a `PallasMessage` type that implements `ChallengeMessage`. Contains bridging code between FROST types and Mina types. |
| `mina-frost-client` | CLI client for distributed key generation (DKG) and signing sessions. Communicates with a `frostd` server. |

**The primary audit scope is the `frost-bluepallas` crate.** Two files in `mina-tx` that implement the `ChallengeMessage` trait and FROST-to-Mina type conversions are also in scope because they directly affect cryptographic correctness.

**Status**: Experimental. Not security audited. Not suitable for use with real funds.

**Rust toolchain**: 1.92.0 (stable), Rust 2021 edition.

**`#![no_std]`**: The `frost-bluepallas` crate is `no_std` (uses `alloc`).

---

### 2. Architecture Overview

#### 2.1 Cryptographic Stack

```
frost-core (v3.0.0-rc.0)        -- Generic FROST protocol engine
    |
    +-- frost-bluepallas          -- Pallas/Poseidon ciphersuite impl
    |       |
    |       +-- lib.rs            -- Ciphersuite definition (BluePallas<M>)
    |       +-- hasher.rs         -- Poseidon hash utilities
    |       +-- keys.rs           -- Key generation/DKG wrappers
    |       +-- negate.rs         -- Y-coordinate negation logic
    |       +-- signing_utilities.rs -- Signing helper flows
    |       +-- errors.rs         -- Error types
    |
    +-- mina-tx (bridging code only)
            +-- pallas_message.rs     -- ChallengeMessage impl, message_hash()
            +-- bluepallas_compat.rs  -- FROST-to-Mina type conversions
```

#### 2.2 FROST Protocol Flow

1. **Key Generation** -- Either trusted dealer (`keys::generate_with_dealer`) or DKG (`keys::dkg::part1` / `part2` / `part3`), both delegating to `frost-core`.
2. **Round 1** -- Each signer generates nonces and commitments (`round1::commit`).
3. **Round 2** -- Each signer generates a signature share (`round2::sign`). Before signing, the `pre_commitment_sign` hook checks the y-coordinate parity of the group commitment and negates nonces/commitments if the y-coordinate is odd (Mina compatibility requirement).
4. **Aggregation** -- Coordinator aggregates shares (`aggregate`). The `pre_commitment_aggregate` hook performs the same parity check from the coordinator's perspective, negating commitments if needed.
5. **Challenge Computation** -- The `BluePallas::challenge()` method delegates to `M::challenge()` (the `ChallengeMessage` trait). For Mina transactions, this is implemented by `PallasMessage` in `mina-tx`, which deserializes the message, hashes it with the public key and nonce commitment x-coordinate using Poseidon, and returns the challenge scalar.

#### 2.3 Mina Compatibility Requirements

- **Y-coordinate evenness**: Mina's Schnorr signature scheme requires the group commitment's y-coordinate to be even. The `pre_commitment_sign` and `pre_commitment_aggregate` hooks enforce this by conditionally negating nonces and commitments.
- **Poseidon hashing**: All FROST hash functions (H1, H3, H4, H5, HDKG, HID) use Poseidon via `mina-hasher::create_legacy`.
- **Challenge computation**: Uses `message_hash()` which constructs a Poseidon hash over `(transaction_input || pub_key_x || pub_key_y || rx)` with a Mina network-specific domain string.
- **H2 is intentionally unimplemented**: The standard FROST H2 (used for challenge computation) is replaced by the Mina-specific challenge path. Calling H2 will panic.

#### 2.4 Key External Dependencies

| Dependency | Version | Role | Security Relevance |
|---|---|---|---|
| `frost-core` | 3.0.0-rc.0 | Generic FROST protocol engine | **Critical** -- all protocol logic lives here. RC status is a risk factor. |
| `mina-curves` | 0.3.0 (git) | Pallas curve definition | **Critical** -- curve arithmetic correctness |
| `mina-hasher` | 0.3.0 (git) | Poseidon hash implementation | **Critical** -- hash function correctness |
| `mina-poseidon` | 0.3.0 (git) | Poseidon constants/parameters | **Critical** -- correct Poseidon parameterisation |
| `mina-signer` | 0.3.0 (git) | Reference signer (used for interop verification in tests) | Medium -- not used in production signing path |
| `ark-ec` | 0.5.0 | Elliptic curve traits and operations | **High** -- curve operations |
| `ark-ff` | 0.5.0 | Finite field arithmetic | **High** -- field operations |
| `ark-serialize` | 0.5.0 | Canonical serialization for curve/field elements | **High** -- serialization correctness |
| `rand_core` | 0.6.4 | Cryptographic RNG trait | **High** -- randomness for nonces |
| `bs58` | 0.5.1 | Base58 encoding | Medium -- address encoding |
| `serde` / `serde_json` | 1.0 | Serialization framework | Medium -- data interchange |
| `sha2` | 0.10 | SHA-256 (used in bs58 checksum, not core signing) | Low |

**Note on `frost-core` version**: The project uses `frost-core` 3.0.0-rc.0, a release candidate. Several integration tests are marked `#[ignore]` due to an upstream bug ([frost-core issue #1015](https://github.com/ZcashFoundation/frost/issues/1015): "signature share verification bug"). This affects `check_sign_with_dkg`, `check_sign_with_dealer`, `check_refresh_shares_with_dealer`, and `check_refresh_shares_with_dkg`.

**Note on `mina-*` dependencies**: These are sourced from `o1-labs/proof-systems` via git tag `0.3.0`. They are not published to crates.io. The audit team may wish to review the specific commit referenced in `Cargo.lock` for known issues.

---

### 3. Audit Scope (In Scope)

#### 3.1 Primary Scope: `frost-bluepallas` Crate (~1,024 lines)

All source files in `frost-bluepallas/src/`:

| File | Lines | Priority | Description |
|---|---|---|---|
| `frost-bluepallas/src/lib.rs` | 354 | **Critical** | Ciphersuite definition (`BluePallas<M>`), `ChallengeMessage` trait, `PallasScalarField` (FROST `Field` impl), `PallasGroup` (FROST `Group` impl), hash function bindings (H1, H3, H4, H5, HDKG, HID), `pre_commitment_sign` and `pre_commitment_aggregate` hooks (y-coordinate parity enforcement), FROST round1/round2/aggregate re-exports. |
| `frost-bluepallas/src/hasher.rs` | 79 | **Critical** | `hash_to_scalar()` and `hash_to_array()` -- Poseidon hash utilities used by all FROST hash functions. `PallasHashElement` wrapper for the `Hashable` trait. Base-to-scalar field conversion. |
| `frost-bluepallas/src/keys.rs` | 192 | **High** | Key generation wrappers: `generate_with_dealer()`, `split()`, and DKG (`dkg::part1`, `part2`, `part3`). All delegate to `frost-core`. Type aliases for `SecretShare`, `SigningShare`, `KeyPackage`, `PublicKeyPackage`, etc. |
| `frost-bluepallas/src/negate.rs` | 191 | **Critical** | `NegateY` trait and implementations for `SigningNonces`, `SigningCommitments`, and `SigningPackage`. Nonce scalar negation and commitment point negation. This module enforces the Mina y-coordinate evenness invariant. |
| `frost-bluepallas/src/signing_utilities.rs` | 144 | **High** | Helper functions `sign_from_packages()`, `generate_signature_random()`, `generate_signature_from_sk()` -- end-to-end signing flows used in testing and potentially by consumers. |
| `frost-bluepallas/src/errors.rs` | 64 | **Low** | `BluePallasError` enum and convenience constructors. No cryptographic logic. |

#### 3.2 Extended Scope: Bridging Code in `mina-tx` (~346 lines)

These files are in scope because they implement the `ChallengeMessage` trait and perform security-critical type conversions between FROST and Mina types. A bug here directly compromises signature correctness.

| File | Lines | Priority | Description |
|---|---|---|---|
| `mina-tx/src/pallas_message.rs` | 262 | **Critical** | `PallasMessage` struct, its `ChallengeMessage` impl (gated behind `frost-bluepallas-compat` feature), `message_hash()` function (Poseidon-based challenge computation), `translate_pk()`, `translate_sig()`, `translate_minask()`, serialization/deserialization. |
| `mina-tx/src/bluepallas_compat.rs` | 84 | **High** | `TryFrom<FrSig<BluePallasSuite>>` for `Sig`, `TryFrom<VerifyingKey<BluePallasSuite>>` for `PubKeySer`, `TransactionEnvelope::to_pallas_message()`, `TransactionSignature::from_frost_signature()`. |

#### 3.3 Security-Relevant Tests

These test files validate cryptographic correctness and should be reviewed for coverage adequacy:

| File | Lines | Purpose |
|---|---|---|
| `frost-bluepallas/tests/integration_tests.rs` | 289 | Runs `frost-core` generic ciphersuite tests against `BluePallas<PallasMessage>`. Includes DKG, share generation, test vector verification, error culprit checks. **Note**: 4 tests are `#[ignore]`d due to upstream frost-core bug. |
| `frost-bluepallas/tests/interoperability_tests.rs` | 17 | Verifies that FROST-generated signatures pass `mina-signer` verification. Runs 256 iterations with different seeds. |
| `frost-bluepallas/tests/recreation_tests.rs` | 178 | Verifies that all FROST types can be decomposed and recreated from their components. |
| `frost-bluepallas/tests/serialization_tests.rs` | 127 | Postcard binary serialization round-trip tests with snapshot assertions. |
| `frost-bluepallas/tests/serde_tests.rs` | 642 | JSON serialization/deserialization tests including invalid input rejection. |
| `frost-bluepallas/tests/common_traits_tests.rs` | 75 | Verifies `Clone`, `Eq`, `Debug` traits on all key types. |
| `frost-bluepallas/tests/deserialize.rs` | 140 | Regression tests for scalar/group element serialization, endianness, identity element rejection. |
| `frost-bluepallas/tests/serialization_pbt.rs` | 50 | Property-based tests (proptest) for scalar and group element round-trip serialization. |
| `frost-bluepallas/tests/helpers/mod.rs` | 30 | Test helper: `verify_signature()` -- converts FROST sig/pk to Mina types and verifies with `mina-signer`. |
| `frost-bluepallas/tests/helpers/samples.rs` | 169 | Generates fixed sample instances of all FROST types for deterministic testing. |
| `frost-bluepallas/tests/helpers/types.rs` | 13 | Type aliases for `BluePallas<PallasMessage>` suite. |

**Test vector files**:
- `frost-bluepallas/tests/helpers/vectors.json`
- `frost-bluepallas/tests/helpers/vectors-big-identifier.json`

**Snapshot files** (11 files in `frost-bluepallas/tests/snapshots/`): Insta snapshot files for postcard serialization regression tests.

---

### 4. Out of Scope

| Component | Reason for Exclusion | Conditions for Bringing In Scope |
|---|---|---|
| `mina-frost-client` crate | CLI/networking code. Does not implement cryptographic primitives. | If the audit budget allows or if the client is found to modify cryptographic data in transit. |
| `mina-tx` crate (excluding the two bridging files listed above) | Transaction types, ZkApp serialization, GraphQL formatting. Not part of the FROST cryptographic path. | If auditors discover that transaction serialization feeds into signing in unexpected ways. |
| `frost-core` (upstream dependency) | Separate open-source project maintained by the Zcash Foundation. Has its own audit history. | If the RC status or the known bug (issue #1015) raises concerns about protocol correctness. The audit team should note that this is an RC version. |
| `mina-curves`, `mina-hasher`, `mina-poseidon` (upstream dependencies) | Maintained by o1-labs. Curve and hash implementations. | If the audit team suspects bugs in Poseidon parameterisation or Pallas curve arithmetic. |
| CI/CD, build scripts, Docker configuration | Infrastructure with no cryptographic impact. | N/A |
| Examples (`frost-bluepallas/examples/`) | Demonstration code, not production. | N/A |
| Dead code / documentation-only files | No runtime impact. | N/A |

---

### 5. Audit Objectives

#### 5.1 Critical Security Goals

1. **Signature unforgeability** -- A threshold signature produced by the FROST protocol must be a valid Mina Schnorr signature that cannot be forged without control of the threshold number of signing shares.
2. **Challenge computation correctness** -- The `ChallengeMessage::challenge()` implementation in `PallasMessage` must produce the same challenge scalar as `mina-signer` for identical inputs. Any deviation means signatures will either fail verification or, worse, pass verification with incorrect semantics.
3. **Y-coordinate parity enforcement** -- The `pre_commitment_sign` and `pre_commitment_aggregate` hooks must correctly enforce even y-coordinates on the group commitment. Incorrect parity handling produces signatures that are invalid on Mina.
4. **Nonce/commitment negation correctness** -- When the group commitment has an odd y-coordinate, all nonces and commitments must be negated consistently across all signers and the coordinator. Inconsistent negation is catastrophic.
5. **Serialization/deserialization correctness** -- Scalars, group elements, and signatures must be serialized in the correct byte order (little-endian for Pallas scalars) and format. Incorrect serialization silently produces invalid cryptographic objects.
6. **Key generation and DKG safety** -- Key splitting, share distribution, and DKG rounds must produce valid Shamir secret shares with correct verification commitments.
7. **Identity element rejection** -- The identity (zero) group element must never be accepted as a valid public key, commitment, or signature component. The `PallasGroup::serialize` and `PallasGroup::deserialize` functions explicitly check for this.
8. **No secret key leakage** -- Error messages, serialization paths, and debug output must not inadvertently expose secret key material.

#### 5.2 Implementation Correctness Checks

1. **Input validation** -- All points, scalars, nonces, commitments, and identifiers must be validated on input. Malformed inputs must be rejected with appropriate errors.
2. **Endianness consistency** -- Scalar serialization uses `ark-serialize`'s compressed format (little-endian). Verify this is consistent throughout the codebase and matches what `mina-signer` expects.
3. **Panic-freedom in library code** -- The `frost-bluepallas` crate is `#![no_std]` library code. There is one `expect()` call in `PallasScalarField::serialize()` at `lib.rs:86` (`"Serialization should not fail for valid scalars"`). Auditors should evaluate whether this invariant truly holds for all inputs or if a panic can be triggered. The `H2` function intentionally panics via `unimplemented!()` -- auditors should verify it cannot be called through any reachable code path.
4. **Hash domain separation** -- The FROST hash functions (H1, H3, H4, H5, HDKG, HID) use domain-separated Poseidon hashing with the prefix `"bluepallas"` concatenated with a purpose tag (`"rho"`, `"nonce"`, `"msg"`, `"com"`, `"dkg"`, `"id"`). Verify that these domain separators are collision-resistant and correctly bound to their context.
5. **Base-to-scalar field conversion** -- `hash_to_scalar()` in `hasher.rs` converts a Poseidon hash output (base field element) to a scalar field element via `Fq::from(hasher.hash(&wrap).into_bigint())`. This cross-field conversion must be verified for correctness (Pallas base field and scalar field have different moduli).
6. **`PallasMessage` deserialization fallback** -- In `pallas_message.rs:234`, the `ChallengeMessage::challenge()` impl uses `unwrap_or_else(|_| Self::from_raw_bytes_default(message))` when deserialization fails. This fallback creates a `PallasMessage` with `NetworkId::TESTNET` and `is_legacy: true`. Auditors should evaluate whether this fallback can be exploited to produce a different challenge hash for the same transaction, enabling signature malleability or domain confusion attacks.
7. **`translate_pk` safety** -- `PubKey::from_point_unsafe()` is used in `pallas_message.rs:141`. The "unsafe" in the name suggests reduced validation. Auditors should verify what checks are skipped.

---

### 6. Specific Areas of Concern

The following items deserve focused auditor attention:

#### 6.1 Y-Coordinate Parity Logic (Critical)

**Files**: `frost-bluepallas/src/lib.rs` (lines 216-268), `frost-bluepallas/src/negate.rs`

The `pre_commitment_sign` and `pre_commitment_aggregate` hooks both:
1. Compute the group commitment via `compute_group_commitment()`
2. Convert to affine coordinates and check `y.into_bigint().is_even()`
3. If odd, negate all nonces/commitments

**Key questions**:
- Is the parity check (`is_even()`) applied to the correct representation of the y-coordinate?
- Are both hooks guaranteed to make the same decision for the same signing package? A mismatch between signer and coordinator would produce an invalid signature.
- Does the negation in `NegateY` for `SigningNonces` (scalar negation) correctly correspond to the negation in `NegateY` for `SigningCommitments` (point negation)?

#### 6.2 Challenge Computation (Critical)

**Files**: `mina-tx/src/pallas_message.rs` (lines 222-241)

The `ChallengeMessage::challenge()` implementation:
1. Translates the FROST verifying key to a Mina `PubKey`
2. Extracts the x-coordinate of the nonce commitment `r`
3. Deserializes the message bytes back into a `PallasMessage` (with fallback)
4. Calls `message_hash()` which constructs a Poseidon hash over `(input || pub_key_x || pub_key_y || rx)`

**Key questions**:
- Does the field element ordering in `message_hash()` match the ordering used by `mina-signer`?
- Is the domain string selection (legacy vs kimchi, mainnet vs testnet) correct and consistent?
- Can the `unwrap_or_else` fallback on deserialization failure lead to a challenge mismatch between signers?

#### 6.3 Serialization Correctness (High)

**Files**: `frost-bluepallas/src/lib.rs` (PallasScalarField, PallasGroup impls)

- `PallasScalarField::serialize` uses `serialize_compressed` which produces little-endian output. The `little_endian_serialize` function returns the same result. Property-based tests confirm this. However, the comment at line 95 says "Parse the canonical 32-byte **big-endian** form" which contradicts the actual little-endian behavior. Auditors should verify which is correct.
- `PallasGroup::Serialization` is `[u8; 96]` (3 base field elements for projective coordinates) but uses `serialize_compressed`. The comment at line 129 notes uncertainty about whether compressed serialization reduces below 96 bytes. Auditors should verify that compressed Pallas points always fit in 96 bytes and that no truncation occurs.

#### 6.4 `frost-core` RC Status and Ignored Tests (High)

Four integration tests are disabled with `#[ignore = "upstream frost-core v3.0.0-rc.0 issue #1015: signature share verification bug"]`:

- `check_sign_with_dkg`
- `check_sign_with_dealer`
- `check_refresh_shares_with_dealer`
- `check_refresh_shares_with_dkg`

These are core protocol correctness tests. The upstream bug affects **signature share verification**, which is a critical security check. Auditors should evaluate the impact of this bug on the overall security of the system.

#### 6.5 `H2` Unimplemented Panic (Medium)

`H2` in `lib.rs:184` panics with `unimplemented!()`. The comment says "H2 is not implemented on purpose, please see the `challenge` function." Auditors should verify that no code path in `frost-core` can reach `H2` when the `challenge()` method is overridden via the `Ciphersuite` trait.

---

### 7. Deliverables

We expect the following deliverables:

1. **Detailed security report**, including:
   - Executive summary
   - Findings categorized by severity (Critical / High / Medium / Low / Informational)
   - Proof-of-concept exploits or demonstrations where applicable
   - Recommended remediations for each finding
   - Assessment of the `frost-core` RC dependency risk
2. **Code review comments** (inline annotations or GitHub PR comments)
3. **Verification of interoperability** -- confirmation that the FROST challenge computation matches `mina-signer` for all transaction types (legacy and kimchi/zkApp)

---

### 8. Preferred Auditor Profile

- Strong Rust cryptography audit experience
- Familiarity with the **FROST threshold Schnorr signature protocol** (RFC 9591 or the ZcashFoundation/frost implementation)
- Understanding of **Pallas/Pasta curves** (curve arithmetic, coordinate conventions, cofactor-1 behavior)
- Experience with **Poseidon hash functions** (sponge construction, domain separation)
- Familiarity with **Mina Protocol signature conventions** (y-coordinate evenness, legacy vs kimchi hashing modes, network ID domain strings)
- Experience auditing `no_std` Rust crates and `ark-*` ecosystem libraries

---

### 9. Appendix: File Inventory

#### Complete list of in-scope source files

```
frost-bluepallas/src/lib.rs                     (354 lines)  -- Ciphersuite definition
frost-bluepallas/src/hasher.rs                   (79 lines)   -- Poseidon hash utilities
frost-bluepallas/src/keys.rs                     (192 lines)  -- Key generation / DKG wrappers
frost-bluepallas/src/negate.rs                   (191 lines)  -- Y-coordinate negation
frost-bluepallas/src/signing_utilities.rs        (144 lines)  -- Signing helpers
frost-bluepallas/src/errors.rs                   (64 lines)   -- Error types
mina-tx/src/pallas_message.rs                    (262 lines)  -- ChallengeMessage impl
mina-tx/src/bluepallas_compat.rs                 (84 lines)   -- FROST-to-Mina conversions
------------------------------------------------------------------------
Total in-scope source:                           ~1,370 lines
```

#### In-scope test files

```
frost-bluepallas/tests/integration_tests.rs      (289 lines)
frost-bluepallas/tests/interoperability_tests.rs  (17 lines)
frost-bluepallas/tests/recreation_tests.rs        (178 lines)
frost-bluepallas/tests/serialization_tests.rs     (127 lines)
frost-bluepallas/tests/serde_tests.rs             (642 lines)
frost-bluepallas/tests/common_traits_tests.rs     (75 lines)
frost-bluepallas/tests/deserialize.rs             (140 lines)
frost-bluepallas/tests/serialization_pbt.rs       (50 lines)
frost-bluepallas/tests/helpers/mod.rs             (30 lines)
frost-bluepallas/tests/helpers/samples.rs         (169 lines)
frost-bluepallas/tests/helpers/types.rs           (13 lines)
------------------------------------------------------------------------
Total test code:                                  ~1,730 lines
```

#### Test vector / snapshot files

```
frost-bluepallas/tests/helpers/vectors.json
frost-bluepallas/tests/helpers/vectors-big-identifier.json
frost-bluepallas/tests/snapshots/                 (11 .snap files)
```

---

### 10. Suggested Audit Focus Order

Ranked by risk and cryptographic criticality:

1. **Y-coordinate parity enforcement** (`lib.rs` pre_commitment hooks + `negate.rs`) -- incorrect parity = invalid Mina signatures
2. **Challenge computation** (`pallas_message.rs` ChallengeMessage impl + `message_hash()`) -- incorrect challenge = forgeable or unverifiable signatures
3. **Field/group serialization** (`lib.rs` PallasScalarField/PallasGroup impls, `deserialize.rs` tests) -- incorrect encoding = silent corruption
4. **FROST-to-Mina type conversions** (`bluepallas_compat.rs`, `translate_pk`, `translate_sig`) -- incorrect conversion = signature/key mismatch
5. **Hash domain separation** (`hasher.rs` + H1/H3/H4/H5/HDKG/HID in `lib.rs`) -- incorrect domain separation = collision risk
6. **Key generation and DKG wrappers** (`keys.rs`) -- largely delegates to frost-core, but verify no custom logic introduces errors
7. **Signing utilities** (`signing_utilities.rs`) -- helper flows, verify they compose correctly
8. **frost-core RC dependency assessment** -- evaluate impact of known bug #1015 and RC status
