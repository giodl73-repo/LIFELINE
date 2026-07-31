# LIFELINE semantic program

## Product achievement

LIFELINE now answers a complete sequence of aggregate household-stability
questions without making eligibility decisions or converting exclusion into
savings.

| Feature | Demonstrated question | Current result |
|---|---|---|
| Benefit continuity | Can earnings rise while resources fall? | One $2,000 Cedar cliff |
| Official scale | What national SNAP scale is source-custodied? | 42.382M average monthly participants; $102.590B FY2025 cost |
| Candidate envelope | What California implementation quantities reconcile? | $86.8M gross administration, -$78.6M offset, $8.2M net |
| Comparison baseline | What predates implementation? | May operations, February error measures, stale churn, aggregate appeals |
| Scenarios | What happens under stress and recovery? | 88.91% → 78.00% → 90.00% synthetic access |
| Realization | Where does a service chain lose continuity? | 100,000 applications → 68,000 receipts → 54,000 renewals |
| Accounting | Are transition costs visible? | $20.2M synthetic net pressure including transition |
| Alternatives | Which bounded paths clear declared floors? | Two of three; none selected |
| Incidence | Who carries burden? | Five groups reconcile; households carry largest burden |
| Delivery | Can the path be implemented? | Six of seven gates; capacity missing |
| Adaptation | Can evidence trigger review without rewriting history? | Immutable successor version 2 created |
| Comparison | Is a definition-matched gap visible? | 609 bps illustrative; not an official peer or target |
| Held handoff | Can Taxlane inspect without inheriting authority? | Complete held pack; zero admission authority |

## Validation

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo run --quiet -- program-held-pack fixtures/synthetic/calfresh-semantic-program.tsv
```

## Remaining evidence work

Observe a complete postimplementation cohort; add current churn, erroneous-
denial, appeals, county capacity, adequacy, and rights evidence; and replace the
illustrative comparison with a custodied definition-matched public comparator.
