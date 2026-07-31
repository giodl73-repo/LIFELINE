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
cargo run --quiet -- official-baseline fixtures/official/usda-fns-snap-2024-2025.tsv
cargo run --quiet -- official-held-pack fixtures/official/usda-fns-snap-2024-2025.tsv
```

The Cedar rows remain deliberately synthetic and use the 2026 HHS poverty
guidelines only as a scale anchor. The official SNAP fixture is a compact
derivation of the FNS national annual summary with its capture checksum. Neither
path makes individual decisions.

## What this proves

- Benefit cliffs and take-up gaps can be measured without merging them.
- Official participation and costs can be replayed without mislabeling
  participation as take-up or a cost change as savings.
- Administrative burden and rights failures remain visible.
- Reduced participation is never automatically labeled efficiency.
- A domain finding can be handed to Taxlane while admission, allocation,
  savings, and rate authority remain false.

## Validate

```powershell
cargo fmt --check
cargo test --all-targets
cargo run --quiet -- analyze fixtures/cedar-benefit-path.tsv
cargo run --quiet -- official-baseline fixtures/official/usda-fns-snap-2024-2025.tsv
```

Official anchor: [HHS 2026 Poverty Guidelines](https://aspe.hhs.gov/topics/poverty-economic-mobility/poverty-guidelines).

## Boundary

LIFELINE is research software. It is not legal advice, an eligibility engine,
an official score, a benefit recommendation, a savings claim, a rate
instruction, or public-release authorization.
