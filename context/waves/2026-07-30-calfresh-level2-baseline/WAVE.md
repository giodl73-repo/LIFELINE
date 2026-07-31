# CalFresh Level 2 access and rights baseline

## Outcome

Establish the current operations, quality, churn, hearing, and implementation
perimeter needed to evaluate California's H.R. 1 CalFresh implementation.

## Result

- exact CDSS raw-workbook checksum and source boundaries recorded;
- May operations and February error measures identified as preimplementation;
- 2020 Q4 churn identified as stale for candidate evaluation;
- FY2025-26 hearing flows preserved without false cohort ratios;
- fourteen-section Level 2 held pack emitted; and
- no eligibility, outcome, savings, admission, allocation, rate, or release
  authority added.

Replay command:
`cargo run --quiet -- level2-held-pack fixtures/official/calfresh-access-rights-floors-2026-07-28.tsv`.
Exact output SHA-256:
`26cc5a2d54dc5ae8ca1389d8632d1f8ec3a3e78f32acd2530834a5f4e3b00c12`.

The Level 2 comparison baseline is complete. Candidate Level 2 remains held
until candidate-coded evidence covers a full recertification cycle.

Fixed point: `pass_with_risk`. Review:
[level2-role-review.md](reviews/level2-role-review.md).
