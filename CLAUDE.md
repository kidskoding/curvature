# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

> **⚠️ DO NOT GENERATE CODE. HINTS ONLY.** This is the user's learning project. Guide with explanations, point at the right crate/method/design decision, sketch signatures in prose, review what they wrote — but do not write implementation code for them. Let them type it.

## What this is

`curvature` — a terminal UI (ratatui) that runs four signature schemes (Ed25519, ECDSA P-256, Schnorr/BIP-340 on secp256k1, BLS12-381) through identical operations and renders the differences live. The point is empirical: fill in a comparison table (sign/verify/batch/aggregate timing, sig & key bytes, aggregatable y/n) by running the ops yourself. It's the concrete answer to "why Ethereum uses BLS for consensus and secp256k1 for transactions."

Status: scaffold only (`main.rs` is hello-world, `Cargo.toml` has no deps). See `TODO.md` for the phased build plan — build in that order.

## Commands

```
cargo run            # launch the TUI
cargo build          # debug build
cargo build --release # release build — USE THIS for any real timing; debug crypto numbers are meaningless
cargo test           # all tests
cargo test <name>    # single test by substring, e.g. cargo test ed25519_roundtrip
cargo test -- --nocapture  # show println output during tests
cargo clippy         # lint
```

Edition is **2024** (`Cargo.toml`) — needs a recent stable toolchain.

## Architecture

One `Scheme` trait is the whole spine. Keys and signatures cross the trait boundary as opaque `Vec<u8>` newtypes (`SecretKey` / `PublicKey` / `Signature`); each scheme downcasts to its own concrete types internally. This is what lets the UI treat all four schemes uniformly and iterate over `&[Box<dyn Scheme>]`.

Trait methods: `keygen`, `sign`, `verify`, `batch_verify` (default impl = verify each), `aggregate → Option`, `verify_aggregate → Option`, `pk_len` / `sig_len`.

Key design decisions (don't undo these):
- **`aggregate` returns `Option`, not `Result`.** Non-aggregatable is a *property* of the scheme, not a runtime error — the UI greys out the aggregation pane instead of showing a failure. Only BLS returns `Some`.
- **Start with common-message aggregation** (`fast_aggregate_verify`) — that's what consensus protocols do and where the speed win lives. Distinct-message aggregation needs a pairing per message and loses most of the advantage; it's out of scope until the common-message case works.
- **Benchmarks run off the UI thread** (`spawn_blocking` or a rayon pool). A 1000-sig BLS aggregate takes real wall-clock time; the render loop must never block on it.

### Panes (each is a view over the schemes)
1. Keygen — sizes side by side
2. Sign/verify — one message, all four, per-op timing
3. Batch — N sigs, individual vs batch verify; Ed25519 batch is ~2–3× faster
4. Aggregation — **BLS only, the headline pane.** N sigs → one 96-byte sig; total-bytes counter stays flat as N→1000 while the naive column climbs linearly
5. Tamper — flip a bit and watch verify fail; also ECDSA malleability (negate `s`, still valid) and the BLS rogue-key attack when proof-of-possession is skipped

## Crypto specifics (easy to get wrong)

- Use **`blst`** for BLS, not `bls12_381` — `blst` is the audited impl Ethereum consensus clients ship. Its API is fiddly; budget time for it (that's why it's a later phase).
- BLS DST = the standard **PoP ciphersuite** (`BLS_SIG_..._POP_`). PoP is also what defends the rogue-key attack demonstrated in the Tamper pane.
- **ECDSA is non-deterministic** unless RFC 6979 is used — surface this in the UI, it's the interesting contrast with Ed25519's determinism.
- Timing numbers are only meaningful in `--release`.
