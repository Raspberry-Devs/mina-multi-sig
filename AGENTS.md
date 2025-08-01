# Agent guidelines for `mina-multi-sig`

## 1. Repository overview

- A Rust workspace consisting of two crates:
  - `frost-bluepallas`: FROST implementation for Mina’s Pallas curve.
  - `frost-client`: CLI utilities (trusted dealer, DKG, signing sessions).
- Code is experimental and **not security audited**. See the security warnings in `README.md` and `frost-bluepallas/README.md`.

## 2. Development workflow

1. **Run tests**
   ```bash
   cargo test
   ```
   The workspace contains extensive tests under `frost-bluepallas/tests` and `frost-client/`.

2. **Lint and formatting**
   - Format all Rust code before committing:
     ```bash
     cargo fmt --all
     ```
   - Run clippy with strict settings (matches `.pre-commit-config.yaml`):
     ```bash
     cargo clippy --all-targets --all-features -- -D clippy::all -W clippy::too_many_arguments
     ```

3. **Pre-commit hooks**
   The project uses [pre-commit](https://pre-commit.com/). Running `pre-commit run --all-files`
   will execute the same formatting and linting checks used in CI.

4. **CI expectations**
   GitHub Actions (see `.github/workflows/rust.yml`) run:
   - `cargo build --verbose`
   - `cargo test --verbose`
   - `cargo fmt --check`

5. **Commit style**
   Commit messages generally follow **Conventional Commits**
   (e.g., `feat:`, `fix:`, `doc:`). Use concise, descriptive messages.

## 3. Coding guidelines

- Rust edition: 2021 (check `rust-toolchain.toml` for toolchain version).
- Prefer descriptive comments and doc comments. Examples and tests are heavily documented—match this style when adding new modules.
- Error handling typically uses `eyre` or `thiserror`; maintain existing patterns.
- `tokio` is used for async code in `frost-client`. Keep async interfaces consistent with existing modules (`session.rs`, `coordinator/` etc.).
- For new features in `frost-bluepallas`, ensure compatibility with Mina’s signature format (see `translate` module) and update tests accordingly.

## 4. Security notes

- The project strives for production-quality code but has not yet undergone a formal security audit. See the README for current disclaimers.
- Avoid storing sensitive key material in version control. Example: `examples/trusted_dealer_example/generated/` is gitignored for this reason.

## 5. File organization hints

- Library code for the FROST implementation: `frost-bluepallas/src/`
- CLI and session logic: `frost-client/src/`
- Examples: `frost-bluepallas/examples/` and `frost-client/examples/`
- Tests: under each crate’s `tests/` directory


## 6. Testing strategy

- Aim for roughly a 50/50 split between unit and integration tests.
- Use unit tests for modules with complex behaviour that benefits from isolated coverage.
- For `frost-client`, prioritize integration tests and only add unit tests for new functionality in future iterations.
- Keep tests lightweight so they run quickly.

## 7. Agile collaboration

- Implement features incrementally and request frequent feedback from maintainers.
- Avoid leaving "TODO" comments for essential functionality unless the implementation would cause unnecessary bloat.
- Keep pull requests small and well scoped. Document significant design decisions in the PR description for reviewers.

## 8. Rust Programming
You are a Rust expert specializing in safe, performant systems programming.
Focus Areas

    Ownership, borrowing, and lifetime annotations
    Trait design and generic programming
    Async/await with Tokio/async-std
    Safe concurrency with Arc, Mutex, channels
    Error handling with Result and custom errors
    FFI and unsafe code when necessary

Approach

    Leverage the type system for correctness
    Zero-cost abstractions over runtime checks
    Explicit error handling - no panics in libraries
    Use iterators over manual loops
    Minimize unsafe blocks with clear invariants

Output

    Idiomatic Rust with proper error handling
    Trait implementations with derive macros
    Async code with proper cancellation
    Unit tests and documentation tests
    Benchmarks with criterion.rs
    Cargo.toml with feature flags

Follow clippy lints. Include examples in doc comments.
