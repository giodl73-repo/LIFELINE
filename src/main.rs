use std::env;
use std::fs;
use std::process::ExitCode;

mod official;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    earnings: i64,
    benefits: i64,
    admin_hours: u64,
    eligible: u64,
    served: u64,
    notice: bool,
    appeal: bool,
}

#[cfg(test)]
mod lane_pack_contract_tests {
    use super::*;

    #[test]
    fn held_pack_exposes_every_taxlane_contract_section() {
        let fixture = include_str!("../fixtures/cedar-benefit-path.tsv");
        let pack = held_pack_json(&analyze(&parse(fixture).unwrap()));
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

#[derive(Debug, PartialEq, Eq)]
struct Analysis {
    steps: usize,
    cliff_count: usize,
    largest_cliff_dollars: i64,
    minimum_takeup_bps: u64,
    maximum_admin_hours: u64,
    rights_floor_pass: bool,
}

fn parse_bool(value: &str, line: usize) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("line {line}: expected true or false, got {value}")),
    }
}

fn parse(input: &str) -> Result<Vec<Step>, String> {
    let mut rows = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line.starts_with("earnings_dollars") {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 7 {
            return Err(format!(
                "line {line_number}: expected 7 tab-separated fields"
            ));
        }
        let number = |field: usize| {
            fields[field]
                .parse::<u64>()
                .map_err(|_| format!("line {line_number}: invalid nonnegative integer"))
        };
        rows.push(Step {
            earnings: number(0)? as i64,
            benefits: number(1)? as i64,
            admin_hours: number(2)?,
            eligible: number(3)?,
            served: number(4)?,
            notice: parse_bool(fields[5], line_number)?,
            appeal: parse_bool(fields[6], line_number)?,
        });
    }
    if rows.len() < 2 {
        return Err("at least two earnings steps are required".into());
    }
    rows.sort_by_key(|row| row.earnings);
    if rows
        .windows(2)
        .any(|pair| pair[0].earnings == pair[1].earnings)
    {
        return Err("earnings steps must be unique".into());
    }
    if rows.iter().any(|row| row.served > row.eligible) {
        return Err("served households cannot exceed eligible households".into());
    }
    Ok(rows)
}

fn analyze(rows: &[Step]) -> Analysis {
    let mut cliff_count = 0;
    let mut largest_cliff = 0;
    for pair in rows.windows(2) {
        let before = pair[0].earnings + pair[0].benefits;
        let after = pair[1].earnings + pair[1].benefits;
        if after < before {
            cliff_count += 1;
            largest_cliff = largest_cliff.max(before - after);
        }
    }
    Analysis {
        steps: rows.len(),
        cliff_count,
        largest_cliff_dollars: largest_cliff,
        minimum_takeup_bps: rows
            .iter()
            .filter(|row| row.eligible > 0)
            .map(|row| row.served * 10_000 / row.eligible)
            .min()
            .unwrap_or(0),
        maximum_admin_hours: rows.iter().map(|row| row.admin_hours).max().unwrap_or(0),
        rights_floor_pass: rows.iter().all(|row| row.notice && row.appeal),
    }
}

fn analysis_json(result: &Analysis) -> String {
    format!(
        "{{\"schema\":\"lifeline.benefit-continuity.v1\",\"steps\":{},\"cliff_count\":{},\"largest_cliff_dollars\":{},\"minimum_takeup_bps\":{},\"maximum_admin_hours\":{},\"rights_floor_pass\":{},\"reduced_participation_is_savings\":false}}",
        result.steps,
        result.cliff_count,
        result.largest_cliff_dollars,
        result.minimum_takeup_bps,
        result.maximum_admin_hours,
        result.rights_floor_pass
    )
}

fn held_pack_json(result: &Analysis) -> String {
    format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"lifeline:cedar-benefit-continuity:v1\",\"track\":\"ISF\",\"domain_repository\":\"LIFELINE\",\"candidate_id\":\"cedar-benefit-continuity\",\"candidate_name\":\"Cedar benefit continuity screen\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"synthetic Cedar\",\"population_or_network\":\"aggregate household earnings path\",\"ownership\":\"illustrative multi-program\",\"time_basis\":\"2026 annual illustration\",\"unit_basis\":\"annual dollars households and basis points\",\"included\":\"cliffs take-up burden notice appeal\",\"excluded\":\"individual eligibility and real program rules\"}},\"source_custody\":{{\"source_id\":\"HHS-POVERTY-2026\",\"publisher\":\"HHS ASPE\",\"source_path_or_url\":\"https://aspe.hhs.gov/topics/poverty-economic-mobility/poverty-guidelines\",\"vintage\":\"2026\",\"capture_status\":\"registry_linked\",\"checksum_or_null\":null}},\"problem\":{{\"baseline_metric\":\"disposable resources by earnings step\",\"baseline_value_or_null\":null,\"affected_population_or_exposure_or_null\":null,\"problem_boundary\":\"synthetic aggregate continuity\",\"cliff_count\":{},\"largest_cliff_dollars\":{},\"minimum_takeup_bps\":{}}},\"intervention\":{{\"mechanism\":null,\"implementing_owner\":null,\"eligibility_rule\":null,\"exclusions\":\"no individual decision\",\"existing_treatment_or_programmed_work\":null}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"effect_population\":\"aggregate households\",\"horizon\":\"annual illustration\",\"uncertainty\":\"not estimated\",\"transferability_boundary\":\"synthetic only\"}},\"service_floors\":{{\"access\":\"take-up reported\",\"quality_safety\":\"not applicable\",\"equity_distribution\":\"not yet evaluated\",\"adequacy_resilience\":\"cliff visible not resolved\",\"delivery_feasibility\":\"not yet evaluated\",\"notice_and_appeal_pass\":{}}},\"costs\":{{\"price_year_or_null\":null,\"gross_cost_or_null\":null,\"implementation_cost_or_null\":null,\"maintenance_cost_or_null\":null,\"offsets_or_null\":null,\"dedicated_receipts_or_null\":null,\"state_local_private_shift_or_null\":null,\"net_cost_or_null\":null,\"public_savings\":null}},\"fiscal_bridge\":{{\"gross_public_funding_need_or_null\":null,\"delivery_efficiency_public_savings_or_null\":null,\"external_economic_benefit_or_null\":null,\"operator_or_private_revenue_or_null\":null,\"legally_dedicated_public_receipts_or_null\":null,\"collection_and_financing_cost_or_null\":null,\"net_public_fiscal_pressure_or_null\":null,\"revenue_authority\":\"none\",\"demand_and_incidence_basis\":\"not established\",\"netting_rule\":\"no values admitted\"}},\"adaptive_pathways\":{{\"pathway_classes\":\"service and administration only\",\"peer_goal_basis\":null,\"evaluation_horizons\":\"annual\",\"realization_owner_or_null\":null,\"transition_and_implementation_cost_or_null\":null,\"uncertainty_and_downside\":\"unbounded\",\"service_floor_and_distribution_result\":\"held\",\"overlap_and_non_additivity\":\"not reconciled\",\"observation_cadence\":null,\"reopen_triggers\":\"official bounded candidate\",\"current_disposition\":\"held\"}},\"delivery\":{{\"capacity\":null,\"schedule\":null,\"milestones\":null,\"useful_life\":null,\"sunset_or_review\":\"review before use\"}},\"overlap\":{{\"shared_projects\":null,\"shared_cost_allocation\":null,\"other_lane_interactions\":\"OAS HLT EDU\",\"non_additivity_rule\":\"no automatic addition\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":false,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"domain_finding_allowed\":true,\"candidate_recommendation_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        result.cliff_count,
        result.largest_cliff_dollars,
        result.minimum_takeup_bps,
        result.rights_floor_pass
    )
}

fn run(args: &[String]) -> Result<String, String> {
    let [command, path] = args else {
        return Err(
            "usage: lifeline <analyze|held-pack|official-baseline|official-held-pack|candidate-baseline|candidate-held-pack|level2-baseline|level2-held-pack|observation-gate> <fixture.tsv>"
                .into(),
        );
    };
    let input = fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    if !input.contains("# source_id=") || !input.contains("# evidence_label=") {
        return Err("fixture must declare source_id and evidence_label".into());
    }
    if command.starts_with("candidate-")
        || command.starts_with("level2-")
        || command.starts_with("observation-")
    {
        return lifeline_calfresh::run(command, &input);
    }
    if command.starts_with("official-") {
        return official::run(command, &input);
    }
    let result = analyze(&parse(&input)?);
    match command.as_str() {
        "analyze" => Ok(analysis_json(&result)),
        "held-pack" => Ok(held_pack_json(&result)),
        _ => Err(format!("unknown command: {command}")),
    }
}

fn main() -> ExitCode {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(2)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXTURE: &str = include_str!("../fixtures/cedar-benefit-path.tsv");

    #[test]
    fn detects_resource_cliff_without_calling_it_savings() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.cliff_count, 1);
        assert_eq!(result.largest_cliff_dollars, 2_000);
        assert!(analysis_json(&result).contains("\"reduced_participation_is_savings\":false"));
    }

    #[test]
    fn reports_takeup_and_burden_separately() {
        let result = analyze(&parse(FIXTURE).unwrap());
        assert_eq!(result.minimum_takeup_bps, 7_500);
        assert_eq!(result.maximum_admin_hours, 18);
    }

    #[test]
    fn rights_failure_blocks_floor() {
        let changed = FIXTURE.replacen("true\ttrue", "false\ttrue", 1);
        assert!(!analyze(&parse(&changed).unwrap()).rights_floor_pass);
    }

    #[test]
    fn rejects_impossible_participation() {
        let changed = FIXTURE.replace("1000\t820", "800\t820");
        assert!(parse(&changed).is_err());
    }

    #[test]
    fn held_pack_preserves_taxlane_authority() {
        let pack = held_pack_json(&analyze(&parse(FIXTURE).unwrap()));
        assert!(pack.contains("\"track\":\"ISF\""));
        assert!(pack.contains("\"public_savings\":null"));
        assert!(pack.contains("\"taxlane_admission_ready\":false"));
        assert!(pack.contains("\"rate_change_allowed\":false"));
    }
}
