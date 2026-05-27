use std::collections::HashMap;
use crate::lab::types::{
    BlockExecution, ExecutionStatus, KernelSession, VerifyCase, VerifyExpectation, VerifyResult,
};

pub struct RunOutcome {
    pub execution: BlockExecution,
    pub session: KernelSession,
    pub elapsed_ms: u32,
}

pub fn run_blueprint(code: &str, session: &KernelSession) -> RunOutcome {
    let start = now_ms();

    let trimmed = code.trim();
    if trimmed.is_empty() {
        return RunOutcome {
            execution: BlockExecution {
                status: ExecutionStatus::Error,
                error: Some("Blueprint is empty.".to_string()),
                ..Default::default()
            },
            session: session.clone(),
            elapsed_ms: now_ms().saturating_sub(start),
        };
    }

    let mut next_session = session.clone();
    next_session.variables = vec![
        ("__status".to_string(), "QUEUED".to_string()),
        ("__bytes".to_string(), trimmed.len().to_string()),
    ];

    let elapsed_ms = now_ms().saturating_sub(start).max(1);
    next_session.last_run_ms = Some(elapsed_ms);

    RunOutcome {
        execution: BlockExecution {
            status: ExecutionStatus::Success,
            stdout: vec![
                "[KERNEL] Local Julia subset kernel pending integration.".to_string(),
                format!("[KERNEL] Received {} bytes of julia code.", trimmed.len()),
                "[KERNEL] Code queued for execution.".to_string(),
            ],
            result: Some("—".to_string()),
            error: None,
            verify_results: Vec::new(),
        },
        session: next_session,
        elapsed_ms,
    }
}

pub fn run_verify(
    cases: &[VerifyCase],
    probes: &HashMap<String, i64>,
    session: &KernelSession,
) -> RunOutcome {
    let start = now_ms();
    let mut results = Vec::new();
    let mut all_passed = true;

    for case in cases {
        let result = evaluate_case(case, probes);
        if !result.passed {
            all_passed = false;
        }
        results.push(result);
    }

    let pass_count = results.iter().filter(|r| r.passed).count();
    let mut next_session = session.clone();
    next_session.variables = vec![
        ("__grader".to_string(), "REFERENCE".to_string()),
        (
            "__tests".to_string(),
            format!("{}/{}", pass_count, results.len()),
        ),
    ];

    let elapsed_ms = now_ms().saturating_sub(start).max(1);
    next_session.last_run_ms = Some(elapsed_ms);

    let status = if all_passed {
        ExecutionStatus::Success
    } else {
        ExecutionStatus::Error
    };

    RunOutcome {
        execution: BlockExecution {
            status,
            stdout: vec![format!(
                "[GRADER] Reference implementation — {}/{} tests passed.",
                pass_count,
                results.len()
            )],
            result: Some(format!("{}/{}", pass_count, results.len())),
            error: if all_passed {
                None
            } else {
                Some(format!("{}/{} tests failed.", results.len() - pass_count, results.len()))
            },
            verify_results: results,
        },
        session: next_session,
        elapsed_ms,
    }
}

fn evaluate_case(case: &VerifyCase, probes: &HashMap<String, i64>) -> VerifyResult {
    let args = match resolve_args(&case.args, probes) {
        Ok(args) => args,
        Err(message) => {
            return VerifyResult {
                expression: case.expression.clone(),
                passed: false,
                expected: format_expectation(&case.expected),
                got: message,
            };
        }
    };

    let outcome = match case.function.as_str() {
        "modInverse" if args.len() == 2 => mod_inverse(args[0], args[1]),
        other => Err(format!("Unsupported function: {other}")),
    };

    let (passed, got) = match (&case.expected, outcome) {
        (VerifyExpectation::Value(expected), Ok(value)) => (*expected == value, value.to_string()),
        (VerifyExpectation::Value(expected), Err(_)) => {
            (false, format!("error (expected {expected})"))
        }
        (VerifyExpectation::Error, Ok(value)) => (false, value.to_string()),
        (VerifyExpectation::Error, Err(_)) => (true, "error".to_string()),
    };

    VerifyResult {
        expression: case.expression.clone(),
        passed,
        expected: format_expectation(&case.expected),
        got,
    }
}

fn resolve_args(
    args: &[crate::lab::types::VerifyArg],
    probes: &HashMap<String, i64>,
) -> Result<Vec<i64>, String> {
    args.iter()
        .map(|arg| match arg {
            crate::lab::types::VerifyArg::Literal(value) => Ok(*value),
            crate::lab::types::VerifyArg::Probe(name) => probes
                .get(name)
                .copied()
                .ok_or_else(|| format!("Unknown probe: {name}")),
        })
        .collect()
}

fn format_expectation(expected: &VerifyExpectation) -> String {
    match expected {
        VerifyExpectation::Value(value) => value.to_string(),
        VerifyExpectation::Error => "error".to_string(),
    }
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = extended_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

fn mod_inverse(a: i64, n: i64) -> Result<i64, String> {
    if n <= 0 {
        return Err("Modulus must be positive.".to_string());
    }
    let (g, x, _) = extended_gcd(a, n);
    if g != 1 {
        return Err("Modular inverse does not exist.".to_string());
    }
    Ok(((x % n) + n) % n)
}

fn now_ms() -> u32 {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now() as u32)
            .unwrap_or(0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lab::types::{VerifyArg, VerifyCase, VerifyExpectation};

    #[test]
    fn mod_inverse_matches_expected_values() {
        assert_eq!(mod_inverse(3, 11), Ok(4));
        assert_eq!(mod_inverse(5, 12), Ok(5));
        assert!(mod_inverse(2, 4).is_err());
    }

    #[test]
    fn verify_runs_probe_linked_cases() {
        let cases = vec![VerifyCase {
            expression: "modInverse(a, n) == 4".to_string(),
            function: "modInverse".to_string(),
            args: vec![VerifyArg::Probe("a".to_string()), VerifyArg::Probe("n".to_string())],
            expected: VerifyExpectation::Value(4),
        }];
        let mut probes = HashMap::new();
        probes.insert("a".to_string(), 3);
        probes.insert("n".to_string(), 11);

        let outcome = run_verify(&cases, &probes, &KernelSession::local_stub());
        assert_eq!(outcome.execution.status, ExecutionStatus::Success);
        assert!(outcome.execution.verify_results[0].passed);
    }
}
