#[derive(Debug, Clone, PartialEq, Eq)]
struct Year {
    fiscal_year: u64,
    participation_thousands: u64,
    average_benefit_cents: u64,
    benefits_thousand_dollars: u64,
    other_costs_thousand_dollars: u64,
    total_costs_thousand_dollars: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct Baseline {
    start_year: u64,
    end_year: u64,
    end_participation_thousands: u64,
    participation_change_thousands: i64,
    participation_change_bps: i64,
    end_average_benefit_cents: u64,
    average_benefit_change_cents: i64,
    average_benefit_change_bps: i64,
    end_total_costs_thousand_dollars: u64,
    total_cost_change_thousand_dollars: i64,
    total_cost_change_bps: i64,
    end_benefit_share_bps: u64,
    end_other_cost_share_bps: u64,
    accounting_difference_thousand_dollars: i64,
}

fn parse(input: &str) -> Result<Vec<Year>, String> {
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("fiscal_year\t") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 6 {
            return Err(format!("line {line_number}: expected 6 fields"));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<u64>()
                .map_err(|_| format!("line {line_number}: invalid nonnegative integer"))
        };
        rows.push(Year {
            fiscal_year: number(0)?,
            participation_thousands: number(1)?,
            average_benefit_cents: number(2)?,
            benefits_thousand_dollars: number(3)?,
            other_costs_thousand_dollars: number(4)?,
            total_costs_thousand_dollars: number(5)?,
        });
    }
    if rows.len() < 2 {
        return Err("at least two official fiscal years are required".into());
    }
    rows.sort_by_key(|row| row.fiscal_year);
    if rows
        .windows(2)
        .any(|pair| pair[0].fiscal_year == pair[1].fiscal_year)
    {
        return Err("official fiscal years must be unique".into());
    }
    Ok(rows)
}

fn change_bps(start: u64, end: u64) -> i64 {
    ((end as i128 - start as i128) * 10_000 / start as i128) as i64
}

fn analyze(rows: &[Year]) -> Baseline {
    let start = &rows[0];
    let end = &rows[rows.len() - 1];
    Baseline {
        start_year: start.fiscal_year,
        end_year: end.fiscal_year,
        end_participation_thousands: end.participation_thousands,
        participation_change_thousands: end.participation_thousands as i64
            - start.participation_thousands as i64,
        participation_change_bps: change_bps(
            start.participation_thousands,
            end.participation_thousands,
        ),
        end_average_benefit_cents: end.average_benefit_cents,
        average_benefit_change_cents: end.average_benefit_cents as i64
            - start.average_benefit_cents as i64,
        average_benefit_change_bps: change_bps(
            start.average_benefit_cents,
            end.average_benefit_cents,
        ),
        end_total_costs_thousand_dollars: end.total_costs_thousand_dollars,
        total_cost_change_thousand_dollars: end.total_costs_thousand_dollars as i64
            - start.total_costs_thousand_dollars as i64,
        total_cost_change_bps: change_bps(
            start.total_costs_thousand_dollars,
            end.total_costs_thousand_dollars,
        ),
        end_benefit_share_bps: end.benefits_thousand_dollars * 10_000
            / end.total_costs_thousand_dollars,
        end_other_cost_share_bps: end.other_costs_thousand_dollars * 10_000
            / end.total_costs_thousand_dollars,
        accounting_difference_thousand_dollars: end.total_costs_thousand_dollars as i64
            - end.benefits_thousand_dollars as i64
            - end.other_costs_thousand_dollars as i64,
    }
}

fn baseline_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"lifeline.official-snap-national-baseline.v1\",\"source_id\":\"USDA-FNS-SNAP-ANNUAL-2025\",\"evidence_label\":\"official_aggregate\",\"start_fiscal_year\":{},\"end_fiscal_year\":{},\"end_average_monthly_participation_thousands\":{},\"participation_change_thousands\":{},\"participation_change_bps\":{},\"end_average_monthly_benefit_cents_per_person\":{},\"average_benefit_change_cents\":{},\"average_benefit_change_bps\":{},\"end_total_costs_thousand_dollars\":{},\"total_cost_change_thousand_dollars\":{},\"total_cost_change_bps\":{},\"end_benefit_share_bps\":{},\"end_other_cost_share_bps\":{},\"accounting_difference_thousand_dollars\":{},\"participation_is_takeup\":false,\"cost_change_is_savings\":false}}",
        result.start_year,
        result.end_year,
        result.end_participation_thousands,
        result.participation_change_thousands,
        result.participation_change_bps,
        result.end_average_benefit_cents,
        result.average_benefit_change_cents,
        result.average_benefit_change_bps,
        result.end_total_costs_thousand_dollars,
        result.total_cost_change_thousand_dollars,
        result.total_cost_change_bps,
        result.end_benefit_share_bps,
        result.end_other_cost_share_bps,
        result.accounting_difference_thousand_dollars
    )
}

fn held_pack_json(result: &Baseline) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"lifeline:usda-fns-snap-2024-2025-national:v1\",\"track\":\"ISF\",\"domain_repository\":\"LIFELINE\",\"candidate_id\":null,\"candidate_name\":null,\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"United States national\",\"population_or_network\":\"SNAP\",\"ownership\":\"USDA Food and Nutrition Service with state administration\",\"time_basis\":\"FY2024 to FY2025\",\"unit_basis\":\"average monthly participants benefits and federal costs\",\"included\":\"published national participation and cost summary\",\"excluded\":\"eligibility take-up households states processing times quality and outcomes\"}},\"source_custody\":{{\"source_id\":\"USDA-FNS-SNAP-ANNUAL-2025\",\"publisher\":\"USDA Food and Nutrition Service\",\"source_path_or_url\":\"https://www.fns.usda.gov/sites/default/files/resource-files/snap-annualsummary-7.pdf\",\"vintage\":\"data as of 2026-07-10\",\"capture_status\":\"derived_with_sha256_in_fixture\",\"checksum_or_null\":\"c5f0903a61e857185b5beb6359974ced4533df3d923f2f1979405ee9be56f9b1\"}},\"problem\":{{\"baseline_metric\":\"national program scale and year-over-year movement\",\"baseline_value_or_null\":{},\"affected_population_or_exposure_or_null\":{},\"problem_boundary\":\"participation is not eligible population or take-up\",\"participation_change_bps\":{},\"total_cost_change_bps\":{}}},\"intervention\":{{\"mechanism\":null,\"implementing_owner\":null,\"eligibility_rule\":null,\"exclusions\":\"no individual eligibility or benefit decision\",\"existing_treatment_or_programmed_work\":null}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"aggregate SNAP program\",\"horizon\":\"two fiscal-year observations\",\"uncertainty\":\"FY2025 data are subject to revision\",\"transferability_boundary\":\"national totals do not describe state delivery\"}},\"service_floors\":{{\"access\":null,\"quality_safety\":null,\"equity_distribution\":null,\"adequacy_resilience\":null,\"delivery_feasibility\":null,\"takeup_bps\":null,\"notice_and_appeal_pass\":null}},\"costs\":{{\"price_year_or_null\":\"current dollars by fiscal year\",\"gross_cost_or_null\":null,\"implementation_cost_or_null\":null,\"maintenance_cost_or_null\":null,\"offsets_or_null\":null,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":null,\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"not established\",\"netting_rule\":\"reported program costs are context not candidate cost\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"baseline observation only\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"annual\",\"realization_owner_or_null\":null,\"transition_and_implementation_cost_or_null\":null,\"uncertainty_and_downside\":\"eligibility and delivery unmeasured\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"SNAP only\",\"observation_cadence\":\"annual\",\"reopen_triggers\":\"reviewed state processing and participation perimeter\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":null,\"schedule\":null,\"milestones\":null,\"useful_life\":null,\"sunset_or_review\":\"refresh after FNS revision\"}},\"overlap\":{{\"shared_projects\":null,\"shared_cost_allocation\":null,\"other_lane_interactions\":\"HLT EDU AGR\",\"non_additivity_rule\":\"do not combine participants with other programs as unique people\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":false,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"eligibility_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.end_total_costs_thousand_dollars,
        result.end_participation_thousands * 1_000,
        result.participation_change_bps,
        result.total_cost_change_bps
    )
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    if !input.contains("# evidence_label=official_aggregate") {
        return Err("official command requires evidence_label=official_aggregate".into());
    }
    let result = analyze(&parse(input)?);
    match command {
        "official-baseline" => Ok(baseline_json(&result)),
        "official-held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown official command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OFFICIAL: &str = include_str!("../fixtures/official/usda-fns-snap-2024-2025.tsv");

    #[test]
    fn calculates_published_national_change_without_calling_it_takeup() {
        let result = analyze(&parse(OFFICIAL).unwrap());
        assert_eq!(result.end_participation_thousands, 42_382);
        assert_eq!(result.participation_change_thousands, 673);
        assert_eq!(result.participation_change_bps, 161);
        assert_eq!(result.end_average_benefit_cents, 18_833);
        assert_eq!(result.average_benefit_change_cents, 157);
        assert_eq!(result.total_cost_change_thousand_dollars, 2_509_570);
        assert_eq!(result.total_cost_change_bps, 250);
        assert_eq!(result.accounting_difference_thousand_dollars, 0);
    }

    #[test]
    fn official_pack_holds_eligibility_service_and_fiscal_claims() {
        let pack = held_pack_json(&analyze(&parse(OFFICIAL).unwrap()));
        assert!(pack.contains("\"takeup_bps\":null"));
        assert!(pack.contains("\"public_savings\":null"));
        assert!(pack.contains("\"candidate_id\":null"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
    }

    #[test]
    fn rejects_duplicate_years() {
        let changed = OFFICIAL.replace("2025\t", "2024\t");
        assert!(parse(&changed).is_err());
    }
}
