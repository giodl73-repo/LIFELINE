# LIFELINE

**Income Security 2.0 — show where earning more can leave a household with
less, without calling lost benefits savings.**

LIFELINE's first product feature reads an aggregate earnings-and-benefits path,
computes disposable resources at every step, detects benefit cliffs, measures
take-up, and blocks promotion when notice or appeal protections are absent.
It then emits a held Taxlane ISF evidence pack whose fiscal fields remain null.

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
```

The source spine anchors the example to the 2026 HHS poverty guidelines. The
program rows are deliberately synthetic because real eligibility varies by
program, household, state, and legal context. The tool is designed for reviewed
aggregate scenarios, never individual decisions.

## What this proves

- Benefit cliffs and take-up gaps can be measured without merging them.
- Administrative burden and rights failures remain visible.
- Reduced participation is never automatically labeled efficiency.
- A domain finding can be handed to Taxlane while admission, allocation,
  savings, and rate authority remain false.

## Validate

```powershell
cargo fmt --check
cargo test --all-targets
cargo run --quiet -- analyze fixtures/cedar-benefit-path.tsv
```

Official anchor: [HHS 2026 Poverty Guidelines](https://aspe.hhs.gov/topics/poverty-economic-mobility/poverty-guidelines).

## Boundary

LIFELINE is research software. It is not legal advice, an eligibility engine,
an official score, a benefit recommendation, a savings claim, a rate
instruction, or public-release authorization.
