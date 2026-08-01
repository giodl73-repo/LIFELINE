use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
struct ObservationGate {
    implementation_start_date: String,
    as_of_date: String,
    elapsed_days: i64,
    minimum_days: i64,
    window_complete: bool,
    source_available: bool,
    candidate_coded: bool,
}

fn days_from_civil(value: &str) -> Result<i64, String> {
    let parts = value
        .split('-')
        .map(|part| {
            part.parse::<i64>()
                .map_err(|_| format!("invalid date {value}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if parts.len() != 3 || !(1..=12).contains(&parts[1]) || !(1..=31).contains(&parts[2]) {
        return Err(format!("invalid date {value}"));
    }
    let mut year = parts[0];
    let month = parts[1];
    let day = parts[2];
    year -= i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let shifted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * shifted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    Ok(era * 146_097 + day_of_era)
}

fn parse(input: &str) -> Result<BTreeMap<String, String>, String> {
    let mut fields = BTreeMap::new();
    for (index, line) in input.lines().enumerate() {
        if line.is_empty() || line.starts_with('#') || line.starts_with("field\t") {
            continue;
        }
        let parts: Vec<_> = line.split('\t').collect();
        if parts.len() != 2 {
            return Err(format!("line {}: expected two fields", index + 1));
        }
        if fields
            .insert(parts[0].to_owned(), parts[1].to_owned())
            .is_some()
        {
            return Err(format!("line {}: duplicate field", index + 1));
        }
    }
    Ok(fields)
}

fn analyze(fields: &BTreeMap<String, String>) -> Result<ObservationGate, String> {
    let required = |key: &str| {
        fields
            .get(key)
            .ok_or_else(|| format!("missing field {key}"))
    };
    let implementation_start_date = required("implementation_start_date")?.to_owned();
    let as_of_date = required("as_of_date")?.to_owned();
    let start = days_from_civil(&implementation_start_date)?;
    let as_of = days_from_civil(&as_of_date)?;
    let minimum_days = required("minimum_observation_days")?
        .parse::<i64>()
        .map_err(|_| "minimum_observation_days must be an integer".to_owned())?;
    let yes = |key: &str| -> Result<bool, String> {
        match required(key)?.as_str() {
            "yes" => Ok(true),
            "no" => Ok(false),
            _ => Err(format!("{key} must be yes or no")),
        }
    };
    if as_of < start || minimum_days <= 0 {
        return Err("observation dates or minimum window are invalid".into());
    }
    Ok(ObservationGate {
        implementation_start_date,
        as_of_date,
        elapsed_days: as_of - start,
        minimum_days,
        window_complete: as_of - start >= minimum_days,
        source_available: yes("postimplementation_source_available")?,
        candidate_coded: yes("postimplementation_records_candidate_coded")?,
    })
}

fn json(gate: &ObservationGate) -> String {
    let review_ready = gate.window_complete && gate.source_available && gate.candidate_coded;
    format!(
        "{{\"schema\":\"lifeline.calfresh-observation-gate.v1\",\"candidate_id\":\"california_hr1_abawd_implementation\",\"as_of_date\":\"{}\",\"implementation_start_date\":\"{}\",\"elapsed_days\":{},\"minimum_observation_days\":{},\"window_complete\":{},\"postimplementation_source_available\":{},\"postimplementation_records_candidate_coded\":{},\"full_gate_review_ready\":{},\"candidate_effect\":null,\"public_savings\":null,\"candidate_admitted\":false,\"disposition\":\"{}\"}}",
        gate.as_of_date,
        gate.implementation_start_date,
        gate.elapsed_days,
        gate.minimum_days,
        gate.window_complete,
        gate.source_available,
        gate.candidate_coded,
        review_ready,
        if review_ready { "ready_for_full_gate_review" } else { "held" }
    )
}

pub(super) fn run(command: &str, input: &str) -> Result<String, String> {
    if !input.contains("# evidence_label=official_postimplementation_source_status") {
        return Err("observation command requires official source-status evidence".into());
    }
    let gate = analyze(&parse(input)?)?;
    match command {
        "observation-gate" => Ok(json(&gate)),
        _ => Err(format!("unknown observation command: {command}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STATUS: &str = include_str!(
        "../../../fixtures/official/calfresh-postimplementation-source-status-2026-08-01.tsv"
    );

    #[test]
    fn computes_the_actual_elapsed_window() {
        let gate = analyze(&parse(STATUS).unwrap()).unwrap();
        assert_eq!(gate.elapsed_days, 61);
        assert!(!gate.window_complete);
    }

    #[test]
    fn unavailable_uncoded_rows_cannot_become_savings() {
        let output = run("observation-gate", STATUS).unwrap();
        assert!(output.contains("\"postimplementation_source_available\":false"));
        assert!(output.contains("\"postimplementation_records_candidate_coded\":false"));
        assert!(output.contains("\"candidate_effect\":null"));
        assert!(output.contains("\"public_savings\":null"));
        assert!(output.contains("\"candidate_admitted\":false"));
    }

    #[test]
    fn all_three_gates_are_required_for_review() {
        let mature = STATUS
            .replace("2026-08-01", "2026-10-01")
            .replace(
                "postimplementation_source_available\tno",
                "postimplementation_source_available\tyes",
            )
            .replace(
                "postimplementation_records_candidate_coded\tno",
                "postimplementation_records_candidate_coded\tyes",
            );
        let output = run("observation-gate", &mature).unwrap();
        assert!(output.contains("\"full_gate_review_ready\":true"));
        assert!(output.contains("\"candidate_admitted\":false"));
        assert!(output.contains("\"disposition\":\"ready_for_full_gate_review\""));
    }
}
