# `verify_signature_share()` does not call `pre_commitment_aggregate()`, causing `InvalidSignatureShare` for ciphersuites that negate commitments

## Summary

`verify_signature_share()` in `frost-core` v3.0.0-rc.0 does not call `<C>::pre_commitment_aggregate()` before computing the group commitment. This causes signature share verification to fail with `InvalidSignatureShare` for any ciphersuite that implements `pre_commitment_aggregate` to transform the signing package (e.g., negating commitments for y-coordinate evenness).

The `aggregate_custom()` function correctly calls `pre_commitment_aggregate()` between binding factor computation and group commitment computation. `verify_signature_share()` skips this step entirely, so it verifies shares against a non-transformed group commitment while the shares themselves were produced using transformed commitments via `pre_commitment_sign()`.

## Affected Version

`frost-core` v3.0.0-rc.0 (crates.io)

## Root Cause

In `frost-core/src/lib.rs`, the `aggregate_custom()` function (line ~633) correctly applies the pre-commitment transformation:

```rust
// In aggregate_custom():
let binding_factor_list: BindingFactorList<C> =
    compute_binding_factor_list(&signing_package, &pubkeys.verifying_key, &[])?;

// ✅ Pre-commitment transformation applied before group commitment
let signing_package = <C>::pre_commitment_aggregate(&signing_package, &binding_factor_list)?;
let group_commitment = compute_group_commitment(&signing_package, &binding_factor_list)?;
```

However, `verify_signature_share()` (line ~748) skips the `pre_commitment_aggregate` call:

```rust
// In verify_signature_share():
let binding_factor_list: BindingFactorList<C> =
    compute_binding_factor_list(&signing_package, verifying_key, &[])?;

// ❌ Missing: <C>::pre_commitment_aggregate(&signing_package, &binding_factor_list)?;
let group_commitment = compute_group_commitment(&signing_package, &binding_factor_list)?;
```

This creates an inconsistency: during signing, `round2::sign()` calls `pre_commitment_sign()` which may negate nonces and commitments to produce the signature share. During verification, `verify_signature_share()` computes the group commitment from the original (non-negated) commitments, so the verification equation does not hold.

### Call Flow Comparison

**`aggregate_custom()` — correct:**
```
pre_aggregate → compute_binding_factor_list → pre_commitment_aggregate → compute_group_commitment → verify
```

**`verify_signature_share()` — missing step:**
```
pre_aggregate → compute_binding_factor_list → compute_group_commitment → verify
                                            ↑
                              pre_commitment_aggregate should be called here
```

## How to Reproduce

Any ciphersuite whose `pre_commitment_aggregate` implementation transforms the signing package (returns `Cow::Owned`) will fail signature share verification. In our case, we implement a Mina/Pallas-based ciphersuite that requires the group commitment's y-coordinate to be even. When it is odd, `pre_commitment_aggregate` negates all signing commitments to enforce evenness.

The signing side (`round2::sign`) correctly applies the matching `pre_commitment_sign` transformation, so shares are computed against negated commitments. But `verify_signature_share` verifies against the original commitments, producing `InvalidSignatureShare`.

Steps:
1. Implement a ciphersuite with a non-trivial `pre_commitment_aggregate` (one that conditionally negates or transforms commitments)
2. Run a full FROST signing flow where the transformation triggers (e.g., group commitment y-coordinate is odd)
3. Call `verify_signature_share()` on any signature share — it returns `Err(Error::InvalidSignatureShare { signer })`

Note: the final aggregate signature produced by `aggregate()` is still correct because `aggregate_custom` does call `pre_commitment_aggregate`. The bug only manifests when `verify_signature_share` is called independently or during cheater detection in `detect_cheater`.

## Suggested Fix

Add a `pre_commitment_aggregate` call to `verify_signature_share()` between binding factor computation and group commitment computation, mirroring what `aggregate_custom()` already does:

```rust
pub fn verify_signature_share<C: Ciphersuite>(
    identifier: Identifier<C>,
    verifying_share: &keys::VerifyingShare<C>,
    signature_share: &round2::SignatureShare<C>,
    signing_package: &SigningPackage<C>,
    verifying_key: &VerifyingKey<C>,
) -> Result<(), Error<C>> {
    // ... pre_aggregate setup (unchanged) ...

    let binding_factor_list: BindingFactorList<C> =
        compute_binding_factor_list(&signing_package, verifying_key, &[])?;

    // Apply pre-commitment transformation (THE FIX)
    let signing_package = <C>::pre_commitment_aggregate(&signing_package, &binding_factor_list)?;

    let group_commitment = compute_group_commitment(&signing_package, &binding_factor_list)?;

    let challenge = <C>::challenge(
        &group_commitment.clone().to_element(),
        verifying_key,
        signing_package.message().as_slice(),
    )?;

    verify_signature_share_precomputed(
        identifier,
        &signing_package,
        &binding_factor_list,
        &group_commitment,
        signature_share,
        verifying_share,
        challenge,
    )
}
```

This is a one-line addition that brings `verify_signature_share` into alignment with `aggregate_custom`. For ciphersuites that use the default (no-op) `pre_commitment_aggregate`, this change is a no-op — the `Cow::Borrowed` path has zero overhead.

## Impact

- Any ciphersuite relying on `pre_commitment_aggregate` to transform commitments cannot use `verify_signature_share` for individual share verification
- Cheater detection in `aggregate_custom` (via `detect_cheater`) also calls `verify_signature_share_precomputed` with the non-transformed signing package, so cheater detection may also be affected
- The `pre_commitment_sign` and `pre_commitment_aggregate` trait methods were designed to work together — this gap breaks the contract between them
