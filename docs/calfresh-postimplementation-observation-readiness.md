# CalFresh postimplementation observation readiness

California began expanded CalFresh work and community-engagement requirements
on June 1, 2026. On July 31, LIFELINE rechecked the official CDSS dashboard and
its raw workbook for the first observable implementation cohort.

| Surface | Latest period | Candidate use |
|---|---|---|
| Core operations | May 2026 | preimplementation comparison only |
| Quality errors | February 2026 | preimplementation comparison only |
| Churn | 2020 Q4 | stale |
| EBT dollars | June 2026 | financial activity, not access or rights evidence |
| Candidate-coded rows | 0 | no effect estimate |
| Full recertification cycles | 0 | no admission review |

The dashboard page's July 28 update label and workbook's July 29 update log are
source-freshness facts. They are not observation dates for every series in the
workbook. The next eligible review begins only when post-June operations can be
separated from the preimplementation baseline; full candidate review still
requires exposure, exemptions, access, accuracy, notice, appeal, county
capacity, household effects, and a complete recertification cycle.

Replay:

```powershell
cargo run --quiet -- observation-readiness fixtures/official/calfresh-postimplementation-observation-readiness-2026-07-31.tsv
cargo run --quiet -- observation-held-pack fixtures/official/calfresh-postimplementation-observation-readiness-2026-07-31.tsv
```

Source: [CDSS CalFresh Data Dashboard](https://www.cdss.ca.gov/inforesources/data-portal/research-and-data/calfresh-data-dashboard).

This is aggregate repository-local research, not an eligibility decision,
candidate effect, savings claim, rate instruction, or release authorization.
