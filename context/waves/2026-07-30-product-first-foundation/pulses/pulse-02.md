# Pulse 02 — official SNAP national baseline

Added a checksum-custodied FY2024-to-FY2025 USDA FNS SNAP baseline. It reports
national participation, average benefit, benefit cost, other federal cost, and
accounting movement while explicitly refusing to equate participation with
eligibility or take-up, or cost movement with performance or savings.

Validation:

```powershell
cargo fmt --check
cargo test --all-targets
cargo run --quiet -- official-baseline fixtures/official/usda-fns-snap-2024-2025.tsv
cargo run --quiet -- official-held-pack fixtures/official/usda-fns-snap-2024-2025.tsv
```
