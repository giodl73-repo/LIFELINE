# LIFELINE

**Income Security 2.0 — show where earning more can leave a household with
less, without calling lost benefits savings.**

LIFELINE's first product feature reads an aggregate earnings-and-benefits path,
computes disposable resources at every step, detects benefit cliffs, measures
take-up, and blocks promotion when notice or appeal protections are absent.
It then emits a held Taxlane ISF evidence pack whose fiscal fields remain null.

Its first official run compares USDA's FY2024 and FY2025 national SNAP
summaries. FY2025 average monthly participation was **42.382 million**, up
**673,000 (1.61%)**; total program costs were **$102.590 billion**, up **2.50%**.
Benefits were 93.36% of that total. These are scale and movement facts—not
eligibility, take-up, adequacy, performance, or savings findings.

The included Cedar example finds a **$2,000 annual resource cliff** when
earnings rise from $20,000 to $30,000: benefits fall faster than earnings rise.
The finding is visible, but the tool neither changes eligibility nor books the
benefit reduction as public savings.

```text
earnings       $20,000 -> $30,000
benefits       $18,000 ->  $6,000
net resources  $38,000 -> $36,000   CLiff: -$2,000
```

## Try it

```powershell
cargo run --quiet -- analyze fixtures/cedar-benefit-path.tsv
cargo run --quiet -- held-pack fixtures/cedar-benefit-path.tsv
cargo run --quiet -- candidate-baseline fixtures/official/california-hr1-abawd-administration-2025-2027.tsv
cargo run --quiet -- candidate-held-pack fixtures/official/california-hr1-abawd-administration-2025-2027.tsv
cargo run --quiet -- official-baseline fixtures/official/usda-fns-snap-2024-2025.tsv
cargo run --quiet -- official-held-pack fixtures/official/usda-fns-snap-2024-2025.tsv
```

The Cedar rows remain deliberately synthetic and use the 2026 HHS poverty
guidelines only as a scale anchor. The official SNAP fixture is a compact
derivation of the FNS national annual summary with its capture checksum. Neither
path makes individual decisions.

## California implementation stress test

LIFELINE now turns California's official H.R. 1 CalFresh estimates into a
replayable candidate envelope. The proposal contains **$86.8 million** of
gross administration work across FY2025-26 and FY2026-27 and a **-$78.6
million** caseload-related administration offset, for a two-year **$8.2
million net administration cost**.

That arithmetic is valid, but it is not an efficiency finding. The same
official record estimates **302,300 affected people** and **$758 million in
lost benefits** in FY2026-27. LIFELINE therefore shows all three quantities
together and keeps public savings null:

| Official proposal quantity | Value | LIFELINE treatment |
|---|---:|---|
| Implementation administration | $86.8M | gross cost |
| Caseload-related administration offset | -$78.6M | accounting offset, not efficiency |
| Net administration | $8.2M | proposed fiscal pressure |
| Affected people, FY2026-27 | 302,300 | access exposure |
| Lost benefits, FY2026-27 | $758M | household impact, never admin savings |

California's FY2024 recertification processing-timeliness baseline was
**88.91%**. The official proposal does not yet supply postimplementation
timeliness, churn, erroneous-denial, appeal-workload, or adequacy results, and
explicitly excludes oral-notice cost. The candidate is consequently bounded
and cost-ready, but held on outcomes, floors, delivery, overlap, and Taxlane
admission.

## What this proves

- Benefit cliffs and take-up gaps can be measured without merging them.
- Official participation and costs can be replayed without mislabeling
  participation as take-up or a cost change as savings.
- Administrative burden and rights failures remain visible.
- Reduced participation is never automatically labeled efficiency.
- Caseload-related administrative offsets remain separate from genuine
  delivery efficiencies and household benefit losses.
- A domain finding can be handed to Taxlane while admission, allocation,
  savings, and rate authority remain false.

## Validate

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo run --quiet -- analyze fixtures/cedar-benefit-path.tsv
cargo run --quiet -- official-baseline fixtures/official/usda-fns-snap-2024-2025.tsv
```

Official anchor: [HHS 2026 Poverty Guidelines](https://aspe.hhs.gov/topics/poverty-economic-mobility/poverty-guidelines).

Candidate sources: [LAO CalFresh County Administration](https://lao.ca.gov/Publications/Report/5149),
[LAO Food Assistance](https://lao.ca.gov/Publications/Report/5126), and
[USDA FY2024 recertification timeliness](https://www.fns.usda.gov/snap/qc/timeliness/rpt-fy24).

## Boundary

LIFELINE is research software. It is not legal advice, an eligibility engine,
an official score, a benefit recommendation, a savings claim, a rate
instruction, or public-release authorization.
