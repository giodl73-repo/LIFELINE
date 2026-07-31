mod level2;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Year {
    fiscal_year: String,
    gross_admin_total_thousand_dollars: i64,
    gross_federal_thousand_dollars: i64,
    gross_state_thousand_dollars: i64,
    gross_county_thousand_dollars: i64,
    caseload_offset_total_thousand_dollars: i64,
    offset_federal_thousand_dollars: i64,
    offset_state_thousand_dollars: i64,
    offset_county_thousand_dollars: i64,
    affected_people: u64,
    lost_benefits_thousand_dollars: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Candidate {
    years: usize,
    gross_admin_total_thousand_dollars: i64,
    caseload_offset_total_thousand_dollars: i64,
    net_admin_total_thousand_dollars: i64,
    federal_net_thousand_dollars: i64,
    state_net_thousand_dollars: i64,
    county_net_thousand_dollars: i64,
    affected_people_fy2026_27: u64,
    lost_benefits_fy2026_27_thousand_dollars: u64,
}

fn parse_number(value: &str, line: usize) -> Result<i64, String> {
    value
        .parse::<i64>()
        .map_err(|_| format!("line {line}: invalid signed integer {value}"))
}

fn parse(input: &str) -> Result<Vec<Year>, String> {
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("fiscal_year\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 11 {
            return Err(format!("line {line_number}: expected 11 fields"));
        }
        let unsigned = |field: usize| {
            fields[field]
                .parse::<u64>()
                .map_err(|_| format!("line {line_number}: invalid nonnegative integer"))
        };
        rows.push(Year {
            fiscal_year: fields[0].to_owned(),
            gross_admin_total_thousand_dollars: parse_number(fields[1], line_number)?,
            gross_federal_thousand_dollars: parse_number(fields[2], line_number)?,
            gross_state_thousand_dollars: parse_number(fields[3], line_number)?,
            gross_county_thousand_dollars: parse_number(fields[4], line_number)?,
            caseload_offset_total_thousand_dollars: parse_number(fields[5], line_number)?,
            offset_federal_thousand_dollars: parse_number(fields[6], line_number)?,
            offset_state_thousand_dollars: parse_number(fields[7], line_number)?,
            offset_county_thousand_dollars: parse_number(fields[8], line_number)?,
            affected_people: unsigned(9)?,
            lost_benefits_thousand_dollars: unsigned(10)?,
        });
    }
    if rows.len() != 2 {
        return Err("candidate requires exactly two fiscal-year rows".into());
    }
    for row in &rows {
        if row.gross_admin_total_thousand_dollars
            != row.gross_federal_thousand_dollars
                + row.gross_state_thousand_dollars
                + row.gross_county_thousand_dollars
        {
            return Err(format!(
                "{} gross funding shares do not reconcile",
                row.fiscal_year
            ));
        }
        if row.caseload_offset_total_thousand_dollars
            != row.offset_federal_thousand_dollars
                + row.offset_state_thousand_dollars
                + row.offset_county_thousand_dollars
        {
            return Err(format!(
                "{} offset funding shares do not reconcile",
                row.fiscal_year
            ));
        }
    }
    Ok(rows)
}

fn analyze(rows: &[Year]) -> Candidate {
    let sum = |value: fn(&Year) -> i64| rows.iter().map(value).sum::<i64>();
    let gross = sum(|row| row.gross_admin_total_thousand_dollars);
    let offset = sum(|row| row.caseload_offset_total_thousand_dollars);
    Candidate {
        years: rows.len(),
        gross_admin_total_thousand_dollars: gross,
        caseload_offset_total_thousand_dollars: offset,
        net_admin_total_thousand_dollars: gross + offset,
        federal_net_thousand_dollars: sum(|row| {
            row.gross_federal_thousand_dollars + row.offset_federal_thousand_dollars
        }),
        state_net_thousand_dollars: sum(|row| {
            row.gross_state_thousand_dollars + row.offset_state_thousand_dollars
        }),
        county_net_thousand_dollars: sum(|row| {
            row.gross_county_thousand_dollars + row.offset_county_thousand_dollars
        }),
        affected_people_fy2026_27: rows[1].affected_people,
        lost_benefits_fy2026_27_thousand_dollars: rows[1].lost_benefits_thousand_dollars,
    }
}

fn baseline_json(result: &Candidate) -> String {
    format!(
        "{{\"schema\":\"lifeline.calfresh-candidate.v1\",\"candidate_id\":\"california_hr1_abawd_implementation\",\"evidence_label\":\"official_state_budget_candidate\",\"legislative_status\":\"federal_law_enacted_state_budget_proposed\",\"fiscal_years\":{},\"gross_admin_total_thousand_dollars\":{},\"caseload_offset_total_thousand_dollars\":{},\"net_admin_total_thousand_dollars\":{},\"federal_net_thousand_dollars\":{},\"state_net_thousand_dollars\":{},\"county_net_thousand_dollars\":{},\"affected_people_fy2026_27\":{},\"lost_benefits_fy2026_27_thousand_dollars\":{},\"california_recertification_timeliness_baseline_bps\":8891,\"caseload_contraction_is_efficiency\":false,\"lost_benefits_are_admin_savings\":false}}",
        result.years,
        result.gross_admin_total_thousand_dollars,
        result.caseload_offset_total_thousand_dollars,
        result.net_admin_total_thousand_dollars,
        result.federal_net_thousand_dollars,
        result.state_net_thousand_dollars,
        result.county_net_thousand_dollars,
        result.affected_people_fy2026_27,
        result.lost_benefits_fy2026_27_thousand_dollars
    )
}

fn held_pack_json(result: &Candidate) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"lifeline:california-hr1-abawd-implementation:v1\",\"track\":\"ISF\",\"domain_repository\":\"LIFELINE\",\"candidate_id\":\"california_hr1_abawd_implementation\",\"candidate_name\":\"California H.R. 1 expanded SNAP work-requirement implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"California\",\"population_or_network\":\"CalFresh recipients newly subject to ABAWD work requirements\",\"ownership\":\"CDSS and county eligibility systems under federal SNAP law\",\"time_basis\":\"FY2025-26 and FY2026-27\",\"unit_basis\":\"thousand nominal dollars and people\",\"included\":\"official proposed administration cost workload offset affected-person and benefit-loss estimates\",\"excluded\":\"individual eligibility decisions postimplementation outcomes and unpriced oral notice\"}},\"source_custody\":{{\"source_id\":\"CA-LAO-CALFRESH-ADMIN-2026-27\",\"publisher\":\"California Legislative Analyst's Office\",\"source_path_or_url\":\"https://lao.ca.gov/Publications/Report/5149\",\"vintage\":\"2026-03-25\",\"capture_status\":\"transcribed_official_tables_with_reconciliation_tests\",\"checksum_or_null\":null,\"companion_source_id\":\"CA-LAO-FOOD-ASSISTANCE-2026-27\"}},\"problem\":{{\"baseline_metric\":\"California SNAP recertification application processing timeliness\",\"baseline_value_or_null\":8891,\"affected_population_or_exposure_or_null\":{},\"problem_boundary\":\"implementation workload and access risk not policy-effect estimate\",\"lost_benefits_fy2026_27_thousand_dollars\":{}}},\"intervention\":{{\"mechanism\":\"implement expanded federal ABAWD work requirements during annual recertification\",\"implementing_owner\":\"CDSS and county welfare departments\",\"eligibility_rule\":\"federal H.R. 1 current-law implementation; LIFELINE does not decide eligibility\",\"exclusions\":\"no person records automated adverse action or recommendation\",\"existing_treatment_or_programmed_work\":\"state budget proposes automation training engagement and discontinuance processing\"}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"302300 affected people estimated in FY2026-27\",\"horizon\":\"implementation through annual recertification\",\"uncertainty\":\"affected groups may overlap and proposal precedes observed implementation outcomes\",\"transferability_boundary\":\"California only\"}},\"service_floors\":{{\"access\":\"FY2024 recertification timeliness baseline 8891 bps; postimplementation result absent\",\"quality_safety\":\"payment accuracy and wrongful exclusion result absent\",\"equity_distribution\":\"subgroup and hardship result absent\",\"adequacy_resilience\":\"projected benefit loss is context not an adequacy finding\",\"delivery_feasibility\":\"oral notice cost excluded and county capacity result absent\",\"notice_and_appeal_pass\":null}},\"costs\":{{\"price_year_or_null\":\"nominal state fiscal years\",\"gross_cost_or_null\":{},\"implementation_cost_or_null\":{},\"maintenance_cost_or_null\":null,\"offsets_or_null\":{},\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":\"federal state and county shares preserved\",\"net_cost_or_null\":{},\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":{},\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":{},\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"official California budget proposal\",\"netting_rule\":\"caseload-related offset may reconcile administration cost but is not delivery efficiency; lost benefits never net into admin savings\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"current-law implementation stress envelope\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"recertification cycle and annual review\",\"realization_owner_or_null\":\"CDSS and counties\",\"transition_and_implementation_cost_or_null\":{},\"uncertainty_and_downside\":\"notice cost and observed access effects absent\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"affected categories may overlap; administrative offset is not separately additive\",\"observation_cadence\":\"monthly county operations and annual federal timeliness\",\"reopen_triggers\":\"postimplementation timeliness churn erroneous-denial appeal workload and cost evidence\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":null,\"schedule\":\"phased through annual recertification\",\"milestones\":null,\"useful_life\":null,\"sunset_or_review\":\"review after one complete recertification cycle\"}},\"overlap\":{{\"shared_projects\":\"CalSAWS automation and county eligibility operations\",\"shared_cost_allocation\":\"official funding shares retained by fiscal year\",\"other_lane_interactions\":\"HLT LAB AGR\",\"non_additivity_rule\":\"do not add benefit loss to administration savings or double-count overlapping affected groups\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"cost_ready\":true,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"eligibility_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.affected_people_fy2026_27,
        result.lost_benefits_fy2026_27_thousand_dollars,
        result.gross_admin_total_thousand_dollars,
        result.gross_admin_total_thousand_dollars,
        result.caseload_offset_total_thousand_dollars,
        result.net_admin_total_thousand_dollars,
        result.gross_admin_total_thousand_dollars,
        result.net_admin_total_thousand_dollars,
        result.gross_admin_total_thousand_dollars
    )
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    if command.starts_with("level2-") {
        return level2::run(command, input);
    }
    if !input.contains("# evidence_label=official_state_budget_candidate") {
        return Err("candidate command requires official_state_budget_candidate evidence".into());
    }
    if !input.contains("# legislative_status=federal_law_enacted_state_budget_proposed") {
        return Err("candidate fixture must state legislative and budget status".into());
    }
    let result = analyze(&parse(input)?);
    match command {
        "candidate-baseline" => Ok(baseline_json(&result)),
        "candidate-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown candidate command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!(
        "../../../fixtures/official/california-hr1-abawd-administration-2025-2027.tsv"
    );

    #[test]
    fn reconciles_costs_without_calling_caseload_contraction_efficiency() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.gross_admin_total_thousand_dollars, 86_800);
        assert_eq!(result.caseload_offset_total_thousand_dollars, -78_600);
        assert_eq!(result.net_admin_total_thousand_dollars, 8_200);
        let output = baseline_json(&result);
        assert!(output.contains("\"caseload_contraction_is_efficiency\":false"));
        assert!(output.contains("\"lost_benefits_are_admin_savings\":false"));
    }

    #[test]
    fn preserves_funding_incidence() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.federal_net_thousand_dollars, 4_100);
        assert_eq!(result.state_net_thousand_dollars, 2_900);
        assert_eq!(result.county_net_thousand_dollars, 1_200);
    }

    #[test]
    fn exposes_people_and_benefits_as_access_context() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.affected_people_fy2026_27, 302_300);
        assert_eq!(result.lost_benefits_fy2026_27_thousand_dollars, 758_000);
    }

    #[test]
    fn held_pack_is_bounded_but_not_admissible() {
        let pack = held_pack_json(&analyze(&parse(FIXTURE).unwrap()));
        assert!(pack.contains("\"candidate_bounded\":true"));
        assert!(pack.contains("\"cost_ready\":true"));
        assert!(pack.contains("\"outcome_ready\":false"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
        assert!(pack.contains("\"public_savings\":null"));
    }

    #[test]
    fn rejects_unreconciled_funding_shares() {
        let changed = FIXTURE.replacen("8400\t4200", "8500\t4200", 1);
        assert!(parse(&changed).is_err());
    }
}
