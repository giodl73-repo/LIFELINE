# CalFresh implementation VERDICT candidate assessment

## Decision and scored object

- Assessment: `lifeline-calfresh-candidate:2026-07-31`.
- Object class: `candidate`.
- Object: California's H.R. 1 CalFresh implementation evidence in LIFELINE at
  `ba39c3cb0f1d06f9326816dc1e38753e8112d0a8`.
- Product owner: LIFELINE; fiscal owner: TAXLANE.
- Not scored: individual eligibility, CalFresh overall, LIFELINE's program
  capability, or the projected benefit loss as a policy outcome.

Scale: `0 missing; 1 designed/partial; 2 executable/bounded; 3 demonstrated`.

| ID | Dimension | Score | Evidence and strength | Principal hold |
|---|---|---:|---|---|
| V | Value | 1 | Gross administration, caseload offset, net cost, affected people, and projected lost benefits remain separate. | No observed household outcome, lifecycle cost, oral-notice cost, or compatible implementation alternative. |
| E | Effectiveness | 1 | Preimplementation timeliness, denial, error, and hearing baselines are fixed. | No candidate-coded postimplementation exposure, service, household, or causal effect. |
| R | Resilience | 1 | Exemptions, waivers, notice, appeals, county capacity, monthly monitoring, and a full-cycle horizon are represented. | No observed churn, correction, recovery, adequacy, or stress-period continuity. |
| D | Deliverability | 2 | Implementation began June 1, 2026; state/county owners, estimated costs, baseline workload, and phased recertification are explicit. | Postimplementation staffing, queues, oral-notice workload, county variation, and realized capacity remain unknown. |
| I | Iteration | 2 | Monthly operations, quarterly hearings, annual federal review, freshness checks, and candidate-coded reopen triggers form an executable loop. | No demonstrated operational correction, outcome learning, rollback, or fiscal rebalancing. |
| C | Coverage and fair access | 2 | Affected population, procedural denials, timeliness, error, appeals, hearings, exemptions, waivers, and protected-group fields stay visible. | Candidate-coded disability, age, veteran, foster-youth, homelessness, race, language, county, notice, appeal, and hardship effects are absent. |
| T | Trust | 3 | Official California/USDA sources, checksummed fixtures, replayable packs, aggregate-only boundaries, role review, and null fiscal claims are demonstrated. | Trust cannot turn lower participation or benefit loss into efficiency. |

Total: **12/21**. Minimum: **1**.

## Gate recommendation

Trust is recommended `required_ready`. The other six dimensions remain
`required_blocked`. The 12/21 total records evidence maturity only and cannot
authorize eligibility action, candidate promotion, savings, or a rate effect.

The candidate may be described as an active current-law implementation with an
$86.8 million gross administration estimate, a -$78.6 million caseload-related
offset, an $8.2 million net administration estimate, 302,300 projected affected
people, and $758 million of projected lost benefits in FY2026-27. The offset is
not delivery efficiency and lost benefits are not savings.

## Next evidence-producing action

Observe one full candidate-coded recertification cycle. Join exposure,
exemption and waiver use, applications, procedural and substantive denials,
timeliness, payment accuracy, churn, notices, appeals, corrections, household
adequacy, protected-group and county incidence, staffing/capacity, and actual
implementation cost. Aggregate data are sufficient; person linkage is neither
required nor authorized.

## `.roles` fixed point

Household, child/family, caseworker, county-operations, labor, public-finance,
due-process, equity, privacy, adequacy, citation, numeracy, and scope lenses
retain 12/21. They recognize an active implementation and unusually strong
access baseline while refusing to call fewer recipients or benefits an
efficiency result. No critical or major documentation finding remains.

Machine record: `fixtures/official/calfresh-verdict.v1.json`.

