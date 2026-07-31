use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Field {
    value: String,
    unit: String,
}

#[derive(Debug, PartialEq, Eq)]
struct ObservationReadiness {
    latest_operations_month: String,
    latest_quality_month: String,
    latest_churn_period: String,
    latest_ebt_month: String,
    postimplementation_operations_rows: u64,
    candidate_coded_rows: u64,
    full_recertification_cycles: u64,
}

fn parse(input: &str) -> Result<BTreeMap<String, Field>, String> {
    let mut fields = BTreeMap::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("field\t") {
            continue;
        }
        let parts: Vec<_> = line.split('\t').collect();
        if parts.len() != 3 {
            return Err(format!("line {line_number}: expected 3 fields"));
        }
        if fields
            .insert(
                parts[0].to_owned(),
                Field {
                    value: parts[1].to_owned(),
                    unit: parts[2].to_owned(),
                },
            )
            .is_some()
        {
            return Err(format!("line {line_number}: duplicate field"));
        }
    }
    Ok(fields)
}

fn text(fields: &BTreeMap<String, Field>, key: &str, unit: &str) -> Result<String, String> {
    let field = fields
        .get(key)
        .ok_or_else(|| format!("missing field {key}"))?;
    if field.unit != unit {
        return Err(format!("field {key} must use {unit}"));
    }
    Ok(field.value.clone())
}

fn number(fields: &BTreeMap<String, Field>, key: &str, unit: &str) -> Result<u64, String> {
    text(fields, key, unit)?
        .parse::<u64>()
        .map_err(|_| format!("field {key} must be a nonnegative integer"))
}

fn analyze(fields: &BTreeMap<String, Field>) -> Result<ObservationReadiness, String> {
    let result = ObservationReadiness {
        latest_operations_month: text(fields, "latest_operations_month", "month")?,
        latest_quality_month: text(fields, "latest_quality_month", "month")?,
        latest_churn_period: text(fields, "latest_churn_period", "quarter")?,
        latest_ebt_month: text(fields, "latest_ebt_month", "month")?,
        postimplementation_operations_rows: number(
            fields,
            "postimplementation_operations_rows",
            "rows",
        )?,
        candidate_coded_rows: number(fields, "candidate_coded_rows", "rows")?,
        full_recertification_cycles: number(fields, "full_recertification_cycles", "cycles")?,
    };
    if result.latest_operations_month >= "2026-06".to_owned()
        && result.postimplementation_operations_rows == 0
    {
        return Err("postimplementation operations month requires observed rows".into());
    }
    if result.candidate_coded_rows > result.postimplementation_operations_rows {
        return Err("candidate-coded rows cannot exceed postimplementation operations rows".into());
    }
    Ok(result)
}

fn readiness_json(result: &ObservationReadiness) -> String {
    format!(
        "{{\"schema\":\"lifeline.calfresh-observation-readiness.v1\",\"candidate_id\":\"california_hr1_abawd_implementation\",\"as_of_date\":\"2026-07-31\",\"implementation_start_date\":\"2026-06-01\",\"source_refresh\":{{\"page_update_label\":\"2026-07-28\",\"workbook_latest_update\":\"2026-07-29\",\"latest_operations_month\":\"{}\",\"latest_quality_month\":\"{}\",\"latest_churn_period\":\"{}\",\"latest_ebt_month\":\"{}\"}},\"postimplementation_operations_rows\":{},\"candidate_coded_rows\":{},\"full_recertification_cycles\":{},\"dashboard_refresh_is_candidate_observation\":false,\"ebt_dollars_are_access_or_rights_evidence\":false,\"first_observation_cohort_available\":false,\"candidate_effect_observable\":false,\"candidate_admitted\":false}}",
        result.latest_operations_month,
        result.latest_quality_month,
        result.latest_churn_period,
        result.latest_ebt_month,
        result.postimplementation_operations_rows,
        result.candidate_coded_rows,
        result.full_recertification_cycles
    )
}

fn held_pack_json(result: &ObservationReadiness) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"lifeline:calfresh-observation-readiness-2026-07-31:v1\",\"track\":\"ISF\",\"domain_repository\":\"LIFELINE\",\"candidate_id\":\"california_hr1_abawd_implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"California\",\"included\":\"official source freshness and aggregate observation availability\",\"excluded\":\"person records eligibility decisions and inferred candidate effects\"}},\"source_custody\":{{\"source_id\":\"CA-CDSS-CALFRESH-OBSERVATION-READINESS-2026-07-31\",\"publisher\":\"California Department of Social Services\",\"source_path_or_url\":\"https://www.cdss.ca.gov/inforesources/data-portal/research-and-data/calfresh-data-dashboard\",\"vintage\":\"checked 2026-07-31; page labels 2026-07-28; workbook log reaches 2026-07-29\",\"checksum_or_null\":\"9f14ce5193b5b6e5fea7086d80bf96469ee4ff16986c578f5ade9f396c446c3a\"}},\"problem\":{{\"baseline_metric\":\"postimplementation observation availability\",\"baseline_value_or_null\":{},\"problem_boundary\":\"operations stop at {}; a source refresh cannot substitute for an observed cohort\"}},\"intervention\":{{\"mechanism\":\"expanded ABAWD requirements beginning 2026-06-01\",\"existing_treatment_or_programmed_work\":\"active current-law implementation\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"uncertainty\":\"no postimplementation operations or candidate-coded cohort is published\"}},\"service_floors\":{{\"access\":null,\"quality_safety\":null,\"equity_distribution\":null,\"adequacy_resilience\":null,\"delivery_feasibility\":null,\"notice_and_appeal_pass\":null}},\"costs\":{{\"gross_cost_or_null\":86800,\"offsets_or_null\":-78600,\"net_cost_or_null\":8200,\"public_savings\":null}},\"fiscal_bridge\":{{\"delivery_efficiency_public_savings_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"netting_rule\":\"source freshness and June EBT activity do not establish savings or candidate effects\"}},\"adaptive_pathways\":{{\"observation_cadence\":\"monthly source refresh\",\"reopen_triggers\":\"candidate-coded access accuracy notice appeal county-capacity and household effects through one full recertification cycle\",\"current_disposition\":\"held_waiting_for_first_postimplementation_operations_cohort\"}},\"delivery\":{{\"schedule\":\"implementation active since 2026-06-01\",\"milestones\":\"first postimplementation operations row then full recertification cycle\"}},\"overlap\":{{\"other_lane_interactions\":\"HLT LAB AGR\",\"non_additivity_rule\":\"EBT dollars and dashboard refreshes are not outcome rows\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"candidate_recommendation_allowed\":false,\"eligibility_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.postimplementation_operations_rows,
        result.latest_operations_month
    )
}

pub(super) fn run(command: &str, input: &str) -> Result<String, String> {
    if !input.contains("# evidence_label=official_postimplementation_observation_readiness")
        || !input.contains("# implementation_start_date=2026-06-01")
    {
        return Err("observation command requires the official timing-bounded snapshot".into());
    }
    let result = analyze(&parse(input)?)?;
    match command {
        "observation-readiness" => Ok(readiness_json(&result)),
        "observation-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown observation command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!(
        "../../../fixtures/official/calfresh-postimplementation-observation-readiness-2026-07-31.tsv"
    );

    #[test]
    fn distinguishes_source_refresh_from_observed_candidate_cohort() {
        let output = run("observation-readiness", FIXTURE).unwrap();
        assert!(output.contains("\"latest_operations_month\":\"2026-05\""));
        assert!(output.contains("\"latest_ebt_month\":\"2026-06\""));
        assert!(output.contains("\"dashboard_refresh_is_candidate_observation\":false"));
        assert!(output.contains("\"first_observation_cohort_available\":false"));
    }

    #[test]
    fn held_pack_preserves_fiscal_and_household_authority() {
        let pack = run("observation-held-pack", FIXTURE).unwrap();
        assert!(pack.contains("\"public_savings\":null"));
        assert!(pack.contains("\"outcome_ready\":false"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
        assert!(pack.contains("\"rate_change_allowed\":false"));
    }

    #[test]
    fn rejects_candidate_rows_without_postimplementation_operations() {
        let changed = FIXTURE.replace("candidate_coded_rows\t0", "candidate_coded_rows\t1");
        assert!(run("observation-readiness", &changed).is_err());
    }
}
