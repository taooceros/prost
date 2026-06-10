# prost AGENTS

Inherits `../AGENTS.md`.

## CRATE JOB
- Own generic protobuf encoding/decoding behavior, including generated-message async encode hooks.
- Keep `prost::Message::encode_raw` synchronous and CPU-compatible.
- Put generated protobuf shape/e2e tests here, including opt-in DSA payload-copy tests that exercise Prost's async sink hook.

## BOUNDARY
- Do not make normal Prost APIs require DSA, Tonic, device paths, or hardware capabilities.
- Hardware-backed DSA tests are gated behind the `hardware-dsa` feature and `TONIC_DSA_WQ`; run them through `dsa_launcher` when direct access lacks capability.
