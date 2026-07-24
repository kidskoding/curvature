# TODO — curvature build plan

Ordered so there's a runnable binary early. Check off tasks individually. Build phases top-to-bottom; don't jump to BLS before the trait and table render.

> Hints-only project: figure out the code yourself. This list says *what* and *in what order*, not *how*.

## Phase 1 — Trait + Ed25519 + table render (goal: working binary, no BLS)

- [ ] Add deps: `ed25519-dalek` (enable `batch`), `rand`, `color-eyre`. Hold off on ratatui until the table renders as plain text.
- [ ] Define opaque newtypes: `SecretKey(Vec<u8>)`, `PublicKey(Vec<u8>)`, `Signature(Vec<u8>)`.
- [ ] Define the `Scheme` trait: `keygen`, `sign`, `verify`, `batch_verify` (default = loop verify), `aggregate → Option`, `verify_aggregate → Option`, `pk_len`, `sig_len`, plus a `name`.
- [ ] Implement `Scheme` for Ed25519 — downcast the `Vec<u8>` newtypes to dalek types internally.
- [ ] Sign/verify roundtrip test for Ed25519 (`cargo test`).
- [ ] Print the comparison table to stdout for a `Vec<Box<dyn Scheme>>` holding just Ed25519. This is the "it runs" checkpoint.

## Phase 2 — More schemes behind the same trait

- [ ] Add `p256`; implement ECDSA P-256. Note: non-deterministic unless RFC 6979 — decide which and remember it for the Tamper/notes pane.
- [ ] Add `k256` (schnorr feature); implement BIP-340 Schnorr.
- [ ] Roundtrip tests for both. Table now shows three columns.

## Phase 3 — TUI shell (ratatui)

- [ ] Add `ratatui` + `crossterm`. Wire `color-eyre` into terminal setup/teardown so a panic restores the terminal.
- [ ] App state + event loop; number keys or tabs switch panes.
- [ ] Pane 1 (Keygen) and Pane 2 (Sign/verify) rendering the trait data. Message input box for signing.
- [ ] Per-op timing: wrap ops in `Instant::now()`, show µs. (Reminder: `--release` for real numbers.)

## Phase 4 — Batch pane

- [ ] Pane 3: generate N sigs, time individual-verify vs `batch_verify`, show µs/sig for each.
- [ ] Confirm Ed25519 batch is ~2–3× faster; throughput curve as N grows.
- [ ] Move benchmarks off the UI thread (`spawn_blocking` / rayon) so the loop stays responsive.

## Phase 5 — BLS + aggregation (the headline; budget an afternoon for blst's API)

- [ ] Add `blst`. Implement `Scheme` for BLS12-381. Use the **PoP ciphersuite DST**.
- [ ] Implement `aggregate` / `verify_aggregate` (only scheme returning `Some`). Start with common-message `fast_aggregate_verify`.
- [ ] Pane 4 (Aggregation, BLS only): N sigs → one 96-byte sig; naive-total-bytes column climbs while aggregate column stays flat to N=1000. Grey the pane out for the other three schemes.

## Phase 6 — Tamper pane

- [ ] Pane 5: bit-flip message / sig / key → watch verify fail.
- [ ] ECDSA malleability demo: negate `s`, signature still verifies.
- [ ] BLS rogue-key attack: show it succeeds when PoP is skipped, fails when enforced.

## Polish (as you go)

- [ ] Release-build reminder surfaced in-UI when timings look suspiciously fast.
- [ ] Notes/legend text explaining ECDSA determinism contrast and why aggregation matters.
