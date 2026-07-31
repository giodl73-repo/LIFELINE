use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
struct Program {
    values: BTreeMap<String, i64>,
}

impl Program {
    fn get(&self, key: &str) -> Result<i64, String> {
        self.values
            .get(key)
            .copied()
            .ok_or_else(|| format!("missing metric: {key}"))
    }

    fn flag(&self, key: &str) -> Result<bool, String> {
        match self.get(key)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(format!("metric {key} must be 0 or 1")),
        }
    }
}

fn parse(input: &str) -> Result<Program, String> {
    for marker in [
        "# source_id=LIFELINE-SYNTHETIC-CALFRESH-SEMANTIC-PROGRAM",
        "# evidence_label=synthetic_aggregate_semantic_program",
    ] {
        if !input.contains(marker) {
            return Err(format!("missing required marker: {marker}"));
        }
    }
    let mut values = BTreeMap::new();
    for (index, line) in input.lines().enumerate() {
        let line_number = index + 1;
        if line.is_empty() || line.starts_with('#') || line == "metric\tvalue" {
            continue;
        }
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 2 {
            return Err(format!("line {line_number}: expected metric and value"));
        }
        let value = fields[1]
            .parse::<i64>()
            .map_err(|_| format!("line {line_number}: invalid integer"))?;
        if values.insert(fields[0].to_owned(), value).is_some() {
            return Err(format!("line {line_number}: duplicate metric"));
        }
    }
    let program = Program { values };
    validate(&program)?;
    Ok(program)
}

fn validate(program: &Program) -> Result<(), String> {
    for key in [
        "baseline_access_bps",
        "stress_access_bps",
        "recovery_access_bps",
        "status_quo_access_bps",
        "notice_support_access_bps",
        "county_capacity_access_bps",
        "observed_access_bps",
        "adaptive_trigger_bps",
        "comparison_access_bps",
    ] {
        if !(0..=10_000).contains(&program.get(key)?) {
            return Err(format!("metric {key} must be basis points"));
        }
    }
    for key in [
        "delivery_owner_named",
        "delivery_capacity_ready",
        "delivery_milestones_ready",
        "delivery_measures_ready",
        "delivery_rights_ready",
        "delivery_stop_condition_ready",
        "delivery_rollback_ready",
        "status_quo_rights_pass",
        "notice_support_rights_pass",
        "county_capacity_rights_pass",
        "comparison_same_definition",
    ] {
        program.flag(key)?;
    }
    let chain = [
        program.get("applications")?,
        program.get("eligible")?,
        program.get("approved")?,
        program.get("benefits_received")?,
        program.get("renewed")?,
    ];
    if chain.iter().any(|value| *value < 0) || chain.windows(2).any(|pair| pair[1] > pair[0]) {
        return Err("realization chain must be nonnegative and nonincreasing".into());
    }
    Ok(())
}

fn scenarios(program: &Program) -> Result<String, String> {
    let baseline = program.get("baseline_access_bps")?;
    let stress = program.get("stress_access_bps")?;
    let recovery = program.get("recovery_access_bps")?;
    Ok(format!(
        "{{\"schema\":\"lifeline.program-scenarios.v1\",\"baseline_access_bps\":{baseline},\"stress_access_bps\":{stress},\"stress_change_bps\":{},\"recovery_access_bps\":{recovery},\"recovery_vs_baseline_bps\":{},\"scenario_versions_immutable\":true,\"observed_candidate_effect\":false}}",
        stress - baseline,
        recovery - baseline
    ))
}

fn realization(program: &Program) -> Result<String, String> {
    let applications = program.get("applications")?;
    let eligible = program.get("eligible")?;
    let approved = program.get("approved")?;
    let received = program.get("benefits_received")?;
    let renewed = program.get("renewed")?;
    let losses = [
        ("eligibility", applications - eligible),
        ("approval", eligible - approved),
        ("payment", approved - received),
        ("renewal", received - renewed),
    ];
    let largest = losses.iter().max_by_key(|(_, loss)| *loss).unwrap();
    Ok(format!(
        "{{\"schema\":\"lifeline.program-realization.v1\",\"applications\":{applications},\"eligible\":{eligible},\"approved\":{approved},\"benefits_received\":{received},\"renewed\":{renewed},\"application_to_payment_bps\":{},\"payment_to_renewal_bps\":{},\"largest_handoff_loss_stage\":\"{}\",\"largest_handoff_loss\":{},\"lost_participation_is_savings\":false}}",
        received * 10_000 / applications,
        renewed * 10_000 / received,
        largest.0,
        largest.1
    ))
}

fn accounting(program: &Program) -> Result<String, String> {
    let gross = program.get("gross_admin_thousand_dollars")?;
    let offset = program.get("caseload_offset_thousand_dollars")?;
    let transition = program.get("transition_thousand_dollars")?;
    Ok(format!(
        "{{\"schema\":\"lifeline.program-accounting.v1\",\"gross_admin_thousand_dollars\":{gross},\"caseload_offset_thousand_dollars\":{offset},\"transition_thousand_dollars\":{transition},\"net_public_fiscal_pressure_thousand_dollars\":{},\"residual_thousand_dollars\":0,\"caseload_offset_is_efficiency\":false,\"public_savings\":null}}",
        gross + offset + transition
    ))
}

fn alternatives(program: &Program) -> Result<String, String> {
    let rows = [
        (
            "status_quo",
            program.get("status_quo_access_bps")?,
            program.flag("status_quo_rights_pass")?,
            program.get("status_quo_cost_thousand_dollars")?,
        ),
        (
            "notice_support",
            program.get("notice_support_access_bps")?,
            program.flag("notice_support_rights_pass")?,
            program.get("notice_support_cost_thousand_dollars")?,
        ),
        (
            "county_capacity",
            program.get("county_capacity_access_bps")?,
            program.flag("county_capacity_rights_pass")?,
            program.get("county_capacity_cost_thousand_dollars")?,
        ),
    ];
    let feasible = rows
        .iter()
        .filter(|(_, access, rights, _)| *access >= 8_500 && *rights)
        .count();
    Ok(format!(
        "{{\"schema\":\"lifeline.program-alternatives.v1\",\"alternative_count\":3,\"feasible_count\":{feasible},\"status_quo\":{{\"access_bps\":{},\"rights_pass\":{},\"cost_thousand_dollars\":{}}},\"notice_support\":{{\"access_bps\":{},\"rights_pass\":{},\"cost_thousand_dollars\":{}}},\"county_capacity\":{{\"access_bps\":{},\"rights_pass\":{},\"cost_thousand_dollars\":{}}},\"selected_alternative\":null}}",
        rows[0].1, rows[0].2, rows[0].3,
        rows[1].1, rows[1].2, rows[1].3,
        rows[2].1, rows[2].2, rows[2].3
    ))
}

fn incidence(program: &Program) -> Result<String, String> {
    let groups = [
        ("households", program.get("household_incidence_points")?),
        ("caseworkers", program.get("caseworker_incidence_points")?),
        ("counties", program.get("county_incidence_points")?),
        ("employers", program.get("employer_incidence_points")?),
        ("taxpayers", program.get("taxpayer_incidence_points")?),
    ];
    let total: i64 = groups.iter().map(|(_, value)| *value).sum();
    if total != 0 {
        return Err("incidence points must reconcile to zero".into());
    }
    let burden = groups.iter().min_by_key(|(_, value)| *value).unwrap();
    Ok(format!(
        "{{\"schema\":\"lifeline.program-incidence.v1\",\"household_points\":{},\"caseworker_points\":{},\"county_points\":{},\"employer_points\":{},\"taxpayer_points\":{},\"reconciliation_points\":{total},\"largest_burden_group\":\"{}\",\"distribution_pass\":false}}",
        groups[0].1, groups[1].1, groups[2].1, groups[3].1, groups[4].1, burden.0
    ))
}

fn delivery(program: &Program) -> Result<String, String> {
    let gates = [
        program.flag("delivery_owner_named")?,
        program.flag("delivery_capacity_ready")?,
        program.flag("delivery_milestones_ready")?,
        program.flag("delivery_measures_ready")?,
        program.flag("delivery_rights_ready")?,
        program.flag("delivery_stop_condition_ready")?,
        program.flag("delivery_rollback_ready")?,
    ];
    let passed = gates.iter().filter(|gate| **gate).count();
    Ok(format!(
        "{{\"schema\":\"lifeline.program-delivery.v1\",\"owner_named\":{},\"capacity_ready\":{},\"milestones_ready\":{},\"measures_ready\":{},\"rights_ready\":{},\"stop_condition_ready\":{},\"rollback_ready\":{},\"gates_passed\":{passed},\"gates_required\":7,\"delivery_ready\":{}}}",
        gates[0], gates[1], gates[2], gates[3], gates[4], gates[5], gates[6], passed == 7
    ))
}

fn adaptive(program: &Program) -> Result<String, String> {
    let observed = program.get("observed_access_bps")?;
    let trigger = program.get("adaptive_trigger_bps")?;
    let current = program.get("current_version")?;
    let triggered = observed < trigger;
    Ok(format!(
        "{{\"schema\":\"lifeline.program-adaptive.v1\",\"current_version\":{current},\"observed_access_bps\":{observed},\"trigger_bps\":{trigger},\"triggered\":{triggered},\"successor_version\":{},\"predecessor_immutable\":true,\"automatic_policy_change\":false}}",
        if triggered { current + 1 } else { current }
    ))
}

fn peers(program: &Program) -> Result<String, String> {
    let current = program.get("baseline_access_bps")?;
    let comparator = program.get("comparison_access_bps")?;
    let same_definition = program.flag("comparison_same_definition")?;
    Ok(format!(
        "{{\"schema\":\"lifeline.program-peers.v1\",\"current_access_bps\":{current},\"illustrative_comparator_access_bps\":{comparator},\"gap_bps\":{},\"same_definition\":{same_definition},\"official_peer_claim\":false,\"automatic_target\":false}}",
        comparator - current
    ))
}

fn held_pack(program: &Program) -> Result<String, String> {
    let net = program.get("gross_admin_thousand_dollars")?
        + program.get("caseload_offset_thousand_dollars")?
        + program.get("transition_thousand_dollars")?;
    let delivery_ready = program.flag("delivery_capacity_ready")?;
    Ok(format!(
        "{{\"schema\":\"taxlane.lane-evidence-pack-candidate.v1\",\"identity\":{{\"pack_id\":\"lifeline:calfresh-semantic-program:v1\",\"track\":\"ISF\",\"domain_repository\":\"LIFELINE\",\"candidate_id\":\"california_hr1_abawd_implementation\",\"fiscal_owner\":\"TAXLANE\"}},\"scope\":{{\"geography\":\"synthetic aggregate demonstration anchored to the California candidate perimeter\",\"included\":\"scenario realization accounting alternatives incidence delivery adaptation and comparison mechanics\",\"excluded\":\"person records eligibility decisions and observed candidate effects\"}},\"source_custody\":{{\"source_id\":\"LIFELINE-SYNTHETIC-CALFRESH-SEMANTIC-PROGRAM\",\"evidence_label\":\"synthetic_aggregate_semantic_program\"}},\"problem\":{{\"baseline_metric\":\"access realization and household stability\",\"baseline_value_or_null\":{}}},\"intervention\":{{\"mechanism\":\"bounded alternatives demonstration\",\"selected_alternative\":null}},\"outcomes\":{{\"bounded_marginal_effect_or_null\":null,\"candidate_effect_observed\":false}},\"service_floors\":{{\"access\":\"stress path below declared floor\",\"rights\":\"independent gate\",\"distribution_pass\":false}},\"costs\":{{\"net_cost_or_null\":{net},\"public_savings\":null}},\"fiscal_bridge\":{{\"net_public_fiscal_pressure_or_null\":null,\"netting_rule\":\"synthetic program accounting cannot enter the fiscal model\"}},\"adaptive_pathways\":{{\"current_disposition\":\"held\",\"automatic_policy_change\":false}},\"delivery\":{{\"delivery_ready\":{delivery_ready},\"capacity_ready\":false}},\"overlap\":{{\"other_lane_interactions\":\"HLT LAB AGR\",\"non_additivity_rule\":\"benefit loss and caseload offsets are not efficiency savings\"}},\"readiness\":{{\"domain_evidence_ready\":true,\"candidate_bounded\":true,\"outcome_ready\":false,\"cost_ready\":false,\"floors_ready\":false,\"delivery_ready\":false,\"overlap_ready\":false,\"taxlane_admission_ready\":false}},\"claim_boundaries\":{{\"candidate_recommendation_allowed\":false,\"eligibility_decision_allowed\":false,\"savings_allowed\":false,\"allocation_allowed\":false,\"rate_change_allowed\":false,\"public_release_allowed\":false}}}}",
        program.get("baseline_access_bps")?
    ))
}

pub fn run(command: &str, input: &str) -> Result<String, String> {
    let program = parse(input)?;
    match command {
        "program-scenarios" => scenarios(&program),
        "program-realization" => realization(&program),
        "program-accounting" => accounting(&program),
        "program-alternatives" => alternatives(&program),
        "program-incidence" => incidence(&program),
        "program-delivery" => delivery(&program),
        "program-adaptive" => adaptive(&program),
        "program-peers" => peers(&program),
        "program-held-pack" => held_pack(&program),
        _ => Err(format!("unknown program command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FIXTURE: &str = include_str!("../../../fixtures/synthetic/calfresh-semantic-program.tsv");

    #[test]
    fn scenario_versions_preserve_stress_and_recovery() {
        let output = run("program-scenarios", FIXTURE).unwrap();
        assert!(output.contains("\"stress_change_bps\":-1091"));
        assert!(output.contains("\"recovery_vs_baseline_bps\":109"));
    }

    #[test]
    fn realization_keeps_renewal_loss_out_of_savings() {
        let output = run("program-realization", FIXTURE).unwrap();
        assert!(output.contains("\"largest_handoff_loss_stage\":\"eligibility\""));
        assert!(output.contains("\"lost_participation_is_savings\":false"));
    }

    #[test]
    fn accounting_includes_transition_and_blocks_savings() {
        let output = run("program-accounting", FIXTURE).unwrap();
        assert!(output.contains("\"net_public_fiscal_pressure_thousand_dollars\":20200"));
        assert!(output.contains("\"public_savings\":null"));
    }

    #[test]
    fn alternatives_preserve_choice() {
        let output = run("program-alternatives", FIXTURE).unwrap();
        assert!(output.contains("\"feasible_count\":2"));
        assert!(output.contains("\"selected_alternative\":null"));
    }

    #[test]
    fn incidence_reconciles_and_exposes_household_burden() {
        let output = run("program-incidence", FIXTURE).unwrap();
        assert!(output.contains("\"reconciliation_points\":0"));
        assert!(output.contains("\"largest_burden_group\":\"households\""));
    }

    #[test]
    fn delivery_fails_on_missing_capacity() {
        let output = run("program-delivery", FIXTURE).unwrap();
        assert!(output.contains("\"gates_passed\":6"));
        assert!(output.contains("\"delivery_ready\":false"));
    }

    #[test]
    fn adaptive_cycle_creates_immutable_successor() {
        let output = run("program-adaptive", FIXTURE).unwrap();
        assert!(output.contains("\"triggered\":true"));
        assert!(output.contains("\"successor_version\":2"));
        assert!(output.contains("\"predecessor_immutable\":true"));
    }

    #[test]
    fn comparison_is_not_promoted_to_official_target() {
        let output = run("program-peers", FIXTURE).unwrap();
        assert!(output.contains("\"gap_bps\":609"));
        assert!(output.contains("\"official_peer_claim\":false"));
        assert!(output.contains("\"automatic_target\":false"));
    }

    #[test]
    fn integrated_pack_remains_held() {
        let output = run("program-held-pack", FIXTURE).unwrap();
        assert!(output.contains("\"taxlane_admission_ready\":false"));
        assert!(output.contains("\"public_savings\":null"));
        assert!(output.contains("\"public_release_allowed\":false"));
    }
}
