use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Metric {
    value: u64,
    unit: String,
}

#[derive(Debug, PartialEq, Eq)]
struct FloorSnapshot {
    households: u64,
    people: u64,
    applications_received: u64,
    applications_approved: u64,
    applications_denied: u64,
    procedural_denials: u64,
    procedural_denial_share_bps: u64,
    thirty_day_timeliness_bps: u64,
    expedited_timeliness_bps: u64,
    negative_error_completed_cases: u64,
    negative_error_rate_bps: u64,
    active_error_rate_bps: u64,
    missed_interview_denials: u64,
    failed_determination_denials: u64,
    procedural_reason_denials: u64,
    churn_30_day_bps: u64,
    churn_90_day_bps: u64,
    appeals_filed: u64,
    appeals_withdrawn: u64,
    hearings_scheduled: u64,
    hearings_held: u64,
    hearings_postponed: u64,
    hearing_nonappearances: u64,
}

fn parse(input: &str) -> Result<BTreeMap<String, Metric>, String> {
    let mut metrics = BTreeMap::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("metric\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 3 {
            return Err(format!("line {line_number}: expected 3 fields"));
        }
        let value = fields[1]
            .parse::<u64>()
            .map_err(|_| format!("line {line_number}: invalid value"))?;
        if metrics
            .insert(
                fields[0].to_owned(),
                Metric {
                    value,
                    unit: fields[2].to_owned(),
                },
            )
            .is_some()
        {
            return Err(format!("line {line_number}: duplicate metric"));
        }
    }
    Ok(metrics)
}

fn value(metrics: &BTreeMap<String, Metric>, key: &str, unit: &str) -> Result<u64, String> {
    let metric = metrics
        .get(key)
        .ok_or_else(|| format!("missing metric {key}"))?;
    if metric.unit != unit {
        return Err(format!("metric {key} must use {unit}"));
    }
    Ok(metric.value)
}

fn analyze(metrics: &BTreeMap<String, Metric>) -> Result<FloorSnapshot, String> {
    let applications_received = value(metrics, "applications_received", "applications")?;
    let applications_approved = value(metrics, "applications_approved", "applications")?;
    let applications_denied = value(metrics, "applications_denied", "applications")?;
    let procedural_denials = value(metrics, "procedural_denials", "applications")?;
    if applications_approved + applications_denied > applications_received
        || procedural_denials > applications_denied
    {
        return Err("application flows exceed their reported bounds".into());
    }
    let basis_points = |key| {
        let result = value(metrics, key, "basis_points")?;
        if result > 10_000 {
            return Err(format!("metric {key} exceeds 10000 basis points"));
        }
        Ok(result)
    };
    Ok(FloorSnapshot {
        households: value(metrics, "calfresh_households", "households")?,
        people: value(metrics, "calfresh_people", "people")?,
        applications_received,
        applications_approved,
        applications_denied,
        procedural_denials,
        procedural_denial_share_bps: procedural_denials * 10_000 / applications_denied,
        thirty_day_timeliness_bps: basis_points("thirty_day_timeliness")?,
        expedited_timeliness_bps: basis_points("expedited_timeliness")?,
        negative_error_completed_cases: value(metrics, "negative_error_completed_cases", "cases")?,
        negative_error_rate_bps: basis_points("negative_error_rate")?,
        active_error_rate_bps: basis_points("active_error_rate")?,
        missed_interview_denials: value(metrics, "missed_interview_denials", "applications")?,
        failed_determination_denials: value(
            metrics,
            "failed_determination_denials",
            "applications",
        )?,
        procedural_reason_denials: value(metrics, "procedural_reason_denials", "applications")?,
        churn_30_day_bps: basis_points("churn_30_day_2020_q4")?,
        churn_90_day_bps: basis_points("churn_90_day_2020_q4")?,
        appeals_filed: value(metrics, "calfresh_appeals_filed_fy2025_26", "appeals")?,
        appeals_withdrawn: value(metrics, "calfresh_appeals_withdrawn_fy2025_26", "appeals")?,
        hearings_scheduled: value(metrics, "calfresh_hearings_scheduled_fy2025_26", "hearings")?,
        hearings_held: value(metrics, "calfresh_hearings_held_fy2025_26", "hearings")?,
        hearings_postponed: value(metrics, "calfresh_hearings_postponed_fy2025_26", "hearings")?,
        hearing_nonappearances: value(
            metrics,
            "calfresh_hearing_nonappearances_fy2025_26",
            "hearings",
        )?,
    })
}

fn baseline_json(snapshot: &FloorSnapshot) -> String {
    format!(
        "{{\"schema\":\"lifeline.calfresh-level2.v1\",\"candidate_id\":\"california_hr1_abawd_implementation\",\"as_of_date\":\"2026-07-28\",\"implementation_start_date\":\"2026-06-01\",\"operations\":{{\"latest_month\":\"2026-05\",\"preimplementation\":true,\"households\":{},\"people\":{},\"applications_received\":{},\"applications_approved\":{},\"applications_denied\":{},\"procedural_denials\":{},\"procedural_denial_share_bps\":{},\"thirty_day_timeliness_bps\":{},\"expedited_timeliness_bps\":{}}},\"quality\":{{\"latest_month\":\"2026-02\",\"preimplementation\":true,\"negative_error_completed_cases\":{},\"negative_error_rate_bps\":{},\"active_error_rate_bps\":{}}},\"denial_reasons\":{{\"missed_interview\":{},\"failed_determination\":{},\"procedural_reason\":{}}},\"churn\":{{\"latest_period\":\"2020-Q4\",\"stale_for_candidate_evaluation\":true,\"thirty_day_bps\":{},\"ninety_day_bps\":{}}},\"hearings_fy2025_26\":{{\"candidate_specific\":false,\"appeals_filed\":{},\"appeals_withdrawn\":{},\"hearings_scheduled\":{},\"hearings_held\":{},\"hearings_postponed\":{},\"nonappearances\":{}}},\"candidate_effect\":null,\"access_floor_pass\":null,\"accuracy_floor_pass\":null,\"notice_appeal_floor_pass\":null,\"county_capacity_floor_pass\":null,\"level2_complete\":false,\"candidate_admitted\":false}}",
        snapshot.households,
        snapshot.people,
        snapshot.applications_received,
        snapshot.applications_approved,
        snapshot.applications_denied,
        snapshot.procedural_denials,
        snapshot.procedural_denial_share_bps,
        snapshot.thirty_day_timeliness_bps,
        snapshot.expedited_timeliness_bps,
        snapshot.negative_error_completed_cases,
        snapshot.negative_error_rate_bps,
        snapshot.active_error_rate_bps,
        snapshot.missed_interview_denials,
        snapshot.failed_determination_denials,
        snapshot.procedural_reason_denials,
        snapshot.churn_30_day_bps,
        snapshot.churn_90_day_bps,
        snapshot.appeals_filed,
        snapshot.appeals_withdrawn,
        snapshot.hearings_scheduled,
        snapshot.hearings_held,
        snapshot.hearings_postponed,
        snapshot.hearing_nonappearances
    )
}

fn held_pack_json(snapshot: &FloorSnapshot) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"lifeline:california-hr1-level2-floor-snapshot:v1\",\"track\":\"ISF\",\"domain_repository\":\"LIFELINE\",\"candidate_id\":\"california_hr1_abawd_implementation\",\"candidate_name\":\"California H.R. 1 expanded SNAP work-requirement implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"California\",\"population_or_network\":\"aggregate CalFresh households applicants and hearing participants\",\"ownership\":\"CDSS county welfare departments and State Hearings Division\",\"time_basis\":\"dashboard through 2026-05 or 2026-02 and hearings FY2025-26\",\"unit_basis\":\"households people applications cases hearings and basis points\",\"included\":\"preimplementation operations quality denial reason stale churn and aggregate hearing baselines\",\"excluded\":\"person records eligibility decisions and candidate-specific postimplementation outcomes\"}},\"source_custody\":{{\"source_id\":\"CA-CALFRESH-ACCESS-RIGHTS-FLOORS-2026-07-28\",\"publisher\":\"California Department of Social Services\",\"source_path_or_url\":\"https://www.cdss.ca.gov/inforesources/data-portal/research-and-data/calfresh-data-dashboard\",\"vintage\":\"2026-07-28\",\"capture_status\":\"official raw workbook transcribed with date and cohort boundaries\",\"checksum_or_null\":\"9f14ce5193b5b6e5fea7086d80bf96469ee4ff16986c578f5ade9f396c446c3a\",\"companion_source_id\":\"CA-SHD-HEARINGS-FY2025-26\"}},\"problem\":{{\"baseline_metric\":\"access accuracy procedural denial churn and hearing floors\",\"baseline_value_or_null\":{{\"applications_received\":{},\"applications_denied\":{},\"procedural_denials\":{},\"thirty_day_timeliness_bps\":{},\"negative_error_rate_bps\":{},\"active_error_rate_bps\":{},\"appeals_filed\":{}}},\"affected_population_or_exposure_or_null\":\"302300 projected people; observed exposure not yet identified\",\"problem_boundary\":\"operations and quality predate implementation; hearings are not candidate-specific; churn is stale\"}},\"intervention\":{{\"mechanism\":\"expanded ABAWD work and community-engagement requirements during recertification\",\"implementing_owner\":\"CDSS and county welfare departments\",\"eligibility_rule\":\"federal current law with exemptions and county waivers; LIFELINE does not decide eligibility\",\"exclusions\":\"no automated adverse action or recommendation\",\"existing_treatment_or_programmed_work\":\"implementation began 2026-06-01 after the latest operational month in this snapshot\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"people newly subject to the expanded requirements\",\"horizon\":\"one complete postimplementation recertification cycle\",\"uncertainty\":\"no matched postimplementation exposure outcome or comparison cohort is published\",\"transferability_boundary\":\"California implementation only\"}},\"service_floors\":{{\"access\":\"May 2026 thirty-day timeliness {} basis points predates implementation\",\"quality_safety\":\"February 2026 negative error {} and active error {} basis points predate implementation\",\"equity_distribution\":\"exemption waiver language disability age veteran foster youth homelessness race language and county results absent\",\"adequacy_resilience\":\"lost benefits remain projected household impact not efficiency\",\"delivery_feasibility\":\"county capacity oral-notice workload and postimplementation queue results absent\",\"notice_and_appeal_pass\":null}},\"costs\":{{\"price_year_or_null\":\"nominal state fiscal years\",\"gross_cost_or_null\":86800,\"implementation_cost_or_null\":86800,\"maintenance_cost_or_null\":null,\"offsets_or_null\":-78600,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":\"federal state and county shares preserved\",\"net_cost_or_null\":8200,\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":86800,\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":8200,\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"official state proposal and preimplementation operating baselines\",\"netting_rule\":\"caseload contraction and lost benefits are not delivery efficiency\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"current-law implementation stress envelope\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"monthly and one full recertification cycle\",\"realization_owner_or_null\":\"CDSS and counties\",\"transition_and_implementation_cost_or_null\":86800,\"uncertainty_and_downside\":\"wrongful exclusion churn notice appeal workload county capacity and household adequacy remain unobserved\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"hearing flows are not a single cohort and affected categories may overlap\",\"observation_cadence\":\"monthly operations plus quarterly hearings\",\"reopen_triggers\":\"candidate-coded postimplementation cohorts through one recertification cycle\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":\"May workload known; postimplementation staffing queue and completion capacity untested\",\"schedule\":\"implementation began 2026-06-01\",\"milestones\":\"first candidate-coded monthly cohort and full recertification cycle\",\"useful_life\":null,\"sunset_or_review\":\"refresh monthly until a full cycle is observed\"}},\"overlap\":{{\"shared_projects\":\"CalSAWS county operations state hearings and employment services\",\"shared_cost_allocation\":\"official federal state and county shares retained\",\"other_lane_interactions\":\"HLT LAB AGR\",\"non_additivity_rule\":\"do not add benefit loss to administrative savings or divide unmatched hearing flows\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"cost_ready\":true,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"eligibility_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        snapshot.applications_received,
        snapshot.applications_denied,
        snapshot.procedural_denials,
        snapshot.thirty_day_timeliness_bps,
        snapshot.negative_error_rate_bps,
        snapshot.active_error_rate_bps,
        snapshot.appeals_filed,
        snapshot.thirty_day_timeliness_bps,
        snapshot.negative_error_rate_bps,
        snapshot.active_error_rate_bps
    )
}

pub(super) fn run(command: &str, input: &str) -> Result<String, String> {
    if !input.contains("# evidence_label=official_calfresh_level2_floor_snapshot") {
        return Err("level2 command requires official CalFresh floor evidence".into());
    }
    if !input.contains("# implementation_start_date=2026-06-01")
        || !input.contains("# latest_operations_month=2026-05")
    {
        return Err("fixture must preserve the implementation timing boundary".into());
    }
    let snapshot = analyze(&parse(input)?)?;
    match command {
        "level2-baseline" => Ok(baseline_json(&snapshot)),
        "level2-held-pack" => Ok(held_pack_json(&snapshot)),
        _ => Err(format!("unknown level2 command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FLOOR_SNAPSHOT: &str =
        include_str!("../../../fixtures/official/calfresh-access-rights-floors-2026-07-28.tsv");

    #[test]
    fn preserves_preimplementation_timing_and_independent_floors() {
        let snapshot = analyze(&parse(FLOOR_SNAPSHOT).unwrap()).unwrap();
        assert_eq!(snapshot.procedural_denial_share_bps, 7_261);
        assert_eq!(snapshot.thirty_day_timeliness_bps, 9_878);
        assert_eq!(snapshot.negative_error_rate_bps, 2_608);
        assert_eq!(snapshot.active_error_rate_bps, 1_080);
    }

    #[test]
    fn stale_churn_and_aggregate_hearings_do_not_become_candidate_effects() {
        let baseline = run("level2-baseline", FLOOR_SNAPSHOT).unwrap();
        assert!(baseline.contains("\"latest_period\":\"2020-Q4\""));
        assert!(baseline.contains("\"stale_for_candidate_evaluation\":true"));
        assert!(baseline.contains("\"candidate_specific\":false"));
        assert!(baseline.contains("\"candidate_effect\":null"));
    }

    #[test]
    fn baseline_keeps_candidate_held() {
        let pack = run("level2-held-pack", FLOOR_SNAPSHOT).unwrap();
        assert!(pack.contains("\"outcome_ready\":false"));
        assert!(pack.contains("\"floors_ready\":false"));
        assert!(pack.contains("\"public_savings\":null"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
    }

    #[test]
    fn rejects_procedural_denials_above_all_denials() {
        let changed = FLOOR_SNAPSHOT.replace("43910\tapplications", "70000\tapplications");
        assert!(run("level2-baseline", &changed).is_err());
    }

    #[test]
    fn held_pack_exposes_every_taxlane_contract_section() {
        let pack = run("level2-held-pack", FLOOR_SNAPSHOT).unwrap();
        for section in [
            "identity",
            "scope",
            "source_custody",
            "problem",
            "intervention",
            "outcomes",
            "service_floors",
            "costs",
            "fiscal_bridge",
            "adaptive_pathways",
            "delivery",
            "overlap",
            "readiness",
            "claim_boundaries",
        ] {
            assert!(
                pack.contains(&format!("\"{section}\":")),
                "missing {section}"
            );
        }
    }
}
