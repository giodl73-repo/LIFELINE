# LIFELINE

**Income Security 2.0 — show where earning more can leave a household with
less, without calling lost benefits savings.**

LIFELINE's first product feature reads an aggregate earnings-and-benefits path,
computes disposable resources at every step, detects benefit cliffs, measures
take-up, and blocks promotion when notice or appeal protections are absent.
It then emits a held Taxlane ISF evidence pack whose fiscal fields remain null.

LIFELINE now carries that first screen through a complete bounded semantic
program. Fifteen executable features cover the household cliff screen,
official SNAP scale, the California implementation candidate and comparison
baseline, scenarios, service realization, accounting, alternatives, incidence,
delivery feasibility, adaptive successors, normalized illustrative comparison,
source-freshness readiness, and integrated held handoffs.

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
cargo run --quiet -- level2-baseline fixtures/official/calfresh-access-rights-floors-2026-07-28.tsv
cargo run --quiet -- level2-held-pack fixtures/official/calfresh-access-rights-floors-2026-07-28.tsv
cargo run --quiet -- observation-readiness fixtures/official/calfresh-postimplementation-observation-readiness-2026-07-31.tsv
cargo run --quiet -- observation-held-pack fixtures/official/calfresh-postimplementation-observation-readiness-2026-07-31.tsv
cargo run --quiet -- program-scenarios fixtures/synthetic/calfresh-semantic-program.tsv
cargo run --quiet -- program-realization fixtures/synthetic/calfresh-semantic-program.tsv
cargo run --quiet -- program-accounting fixtures/synthetic/calfresh-semantic-program.tsv
cargo run --quiet -- program-alternatives fixtures/synthetic/calfresh-semantic-program.tsv
cargo run --quiet -- program-incidence fixtures/synthetic/calfresh-semantic-program.tsv
cargo run --quiet -- program-delivery fixtures/synthetic/calfresh-semantic-program.tsv
cargo run --quiet -- program-adaptive fixtures/synthetic/calfresh-semantic-program.tsv
cargo run --quiet -- program-peers fixtures/synthetic/calfresh-semantic-program.tsv
cargo run --quiet -- program-held-pack fixtures/synthetic/calfresh-semantic-program.tsv
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

## Level 2 access and rights baseline

California began the expanded work and community-engagement rules on **June 1,
2026**. The latest CDSS operational dashboard values are from May, however, and
the latest error measures are from February. They therefore establish a
preimplementation baseline—not observed H.R. 1 effects.

The May snapshot covers **3,107,208 households and 5,166,532 people**. It
reports 161,045 applications received, 60,469 denied, and 43,910 procedural
denials; within those dashboard fields, procedural denials equal **72.61%** of
all denials. Thirty-day processing timeliness was **98.78%** and expedited
timeliness was **96.52%**. These quantities expose workload and access risk,
but do not show that any denial was wrongful.

The official FY2025-26 State Hearings report records **44,504 CalFresh appeals
filed** and **10,434 hearings held**. It aggregates all CalFresh issues, and its
final quarter contains only one implementation month, so it cannot measure the
candidate's notice or appeal effect. The dashboard's latest statewide churn
period is 2020 Q4 and is too stale for candidate evaluation. LIFELINE has
therefore completed the Level 2 comparison baseline while keeping access,
accuracy, churn, rights, county capacity, and candidate admission held.

### First postimplementation observation check

The official dashboard was checked again on July 31. Its page labels the source
updated July 28, while the workbook's own update log reaches July 29. Neither
date means that a June implementation cohort is present: core household,
person, application, and timeliness operations still stop in **May 2026**, and
quality measures stop in **February 2026**. The workbook includes June EBT
dollars, but a payment aggregate cannot establish access, accuracy, notice,
appeal, county capacity, or an H.R. 1 effect.

The executable observation check therefore reports **zero postimplementation
operations rows, zero candidate-coded rows, and zero complete recertification
cycles**. It makes the next reopening event exact while preventing a refreshed
file or a later-dated financial series from being mistaken for outcome
evidence.

An [August 1 source recheck](docs/calfresh-postimplementation-source-status-2026-08-01.md)
adds two useful boundaries: the relevant STAT 47 files are temporarily removed
during a state de-identification update, and implementation advisory materials
are not candidate-coded outcomes. Sixty-one elapsed days, source availability,
and cohort coding remain separate facts; none creates a savings value.

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
- Preimplementation operations and aggregate hearings can be fixed as future
comparison surfaces without being mislabeled as candidate outcomes.
- Stale churn data and uncoded hearing flows remain explicit evidence gaps.

## Complete semantic-program demonstration

The program fixture is synthetic and aggregate, but its mechanics are
consequential. A stress path takes access from 88.91% to 78.00%; recovery rises
to 90.00% without rewriting either predecessor. An illustrative 100,000-
application chain reaches 68,000 benefit receipts and 54,000 renewals. Its
largest handoff loss is eligibility, and every loss remains distinct from
savings.

Accounting combines $86.8M of gross administration, a -$78.6M caseload offset,
and $12.0M of illustrative transition work into $20.2M of net public pressure.
The transition amount is synthetic and cannot enter Taxlane. It exists to
prove that an implementation comparison cannot omit transition cost.

Three alternatives expose different access, rights, and cost results. Two pass
the declared synthetic access-and-rights screen, but LIFELINE selects neither.
The incidence map reconciles five stakeholder groups to zero points while
showing households as the largest burden carrier. Six of seven delivery gates
pass; missing county capacity keeps delivery held. The adaptive rule then
creates immutable version 2 because observed synthetic access falls below its
declared trigger.

The comparison feature uses a definition-matched **illustrative** 95% value.
It is deliberately not called an official peer or policy target. Replacing it
with a custodied public comparator is the next corpus task, not a condition for
replaying the semantic mechanics.

## Validate

```powershell
cargo fmt --check
cargo test --workspace --all-targets
cargo run --quiet -- analyze fixtures/cedar-benefit-path.tsv
cargo run --quiet -- official-baseline fixtures/official/usda-fns-snap-2024-2025.tsv
cargo run --quiet -- level2-baseline fixtures/official/calfresh-access-rights-floors-2026-07-28.tsv
cargo run --quiet -- observation-readiness fixtures/official/calfresh-postimplementation-observation-readiness-2026-07-31.tsv
cargo run --quiet -- program-held-pack fixtures/synthetic/calfresh-semantic-program.tsv
```

Official anchor: [HHS 2026 Poverty Guidelines](https://aspe.hhs.gov/topics/poverty-economic-mobility/poverty-guidelines).

Candidate sources: [LAO CalFresh County Administration](https://lao.ca.gov/Publications/Report/5149),
[LAO Food Assistance](https://lao.ca.gov/Publications/Report/5126), and
[USDA FY2024 recertification timeliness](https://www.fns.usda.gov/snap/qc/timeliness/rpt-fy24).

Level 2 baseline sources: [CDSS CalFresh Data Dashboard](https://www.cdss.ca.gov/inforesources/data-portal/research-and-data/calfresh-data-dashboard),
[CDSS FY2025-26 Hearing Data Report](https://www.cdss.ca.gov/Portals/9/SHD/SHD%20Hearing%20Data%20Summary%20Report%20FY%202025-2026.pdf),
and [CDSS work and community-engagement requirements](https://www.cdss.ca.gov/inforesources/calfresh/abawd).

## Boundary

LIFELINE is research software. It is not legal advice, an eligibility engine,
an official score, a benefit recommendation, a savings claim, a rate
instruction, or public-release authorization.
