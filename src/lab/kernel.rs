use std::collections::HashMap;

use crate::lab::crypto;
use crate::lab::julia_interp;
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
    let elapsed_ms;

    match julia_interp::execute(trimmed) {
        Ok((stdout, result)) => {
            elapsed_ms = now_ms().saturating_sub(start).max(1);
            next_session.variables = vec![
                ("__status".to_string(), "OK".to_string()),
                ("__lines".to_string(), stdout.len().to_string()),
            ];
            next_session.last_run_ms = Some(elapsed_ms);

            RunOutcome {
                execution: BlockExecution {
                    status: ExecutionStatus::Success,
                    stdout,
                    result,
                    error: None,
                    verify_results: Vec::new(),
                },
                session: next_session,
                elapsed_ms,
            }
        }
        Err(message) => {
            elapsed_ms = now_ms().saturating_sub(start).max(1);
            next_session.variables = vec![("__status".to_string(), "ERROR".to_string())];
            next_session.last_run_ms = Some(elapsed_ms);

            RunOutcome {
                execution: BlockExecution {
                    status: ExecutionStatus::Error,
                    stdout: Vec::new(),
                    result: None,
                    error: Some(message),
                    verify_results: Vec::new(),
                },
                session: next_session,
                elapsed_ms,
            }
        }
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
                "[GRADER] Reference implementation: {}/{} tests passed.",
                pass_count,
                results.len()
            )],
            result: Some(format!("{}/{}", pass_count, results.len())),
            error: if all_passed {
                None
            } else {
                Some(format!(
                    "{}/{} tests failed.",
                    results.len() - pass_count,
                    results.len()
                ))
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

    let outcome = match (case.function.as_str(), args.len()) {
        ("modInverse", 2) => crypto::mod_inverse(args[0], args[1]),
        ("powermod", 3) => crypto::powermod(args[0], args[1], args[2]),
        ("crt_two", 4) => crypto::crt_two(args[0], args[1], args[2], args[3]),
        ("euler_phi", 1) => crypto::euler_phi(args[0]),
        ("fermat_check", 2) => crypto::fermat_check(args[0], args[1]),
        ("is_prime", 1) => crypto::is_prime(args[0]),
        ("prime_count", 1) => crypto::prime_count(args[0]),
        ("nth_prime", 1) => crypto::nth_prime(args[0]),
        ("isprime_check", 1) => crypto::isprime_check(args[0]),
        ("rsa_phi", 2) => crypto::rsa_phi(args[0], args[1]),
        ("rsa_ed_check", 3) => crypto::rsa_ed_check(args[0], args[1], args[2]),
        ("rsa_encrypt", 3) => crypto::rsa_encrypt(args[0], args[1], args[2]),
        ("rsa_decrypt", 3) => crypto::rsa_decrypt(args[0], args[1], args[2]),
        (other, _) => Err(format!("Unsupported function: {other}")),
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
        assert_eq!(crypto::mod_inverse(3, 11), Ok(4));
        assert_eq!(crypto::mod_inverse(5, 12), Ok(5));
        assert!(crypto::mod_inverse(2, 4).is_err());
    }

    #[test]
    fn powermod_matches_expected_values() {
        assert_eq!(crypto::powermod(2, 10, 13), Ok(10));
        assert_eq!(crypto::powermod(3, 13, 97), Ok(31));
        assert_eq!(crypto::powermod(2, 1000, 13), Ok(3));
    }

    #[test]
    fn crt_two_matches_expected_values() {
        assert_eq!(crypto::crt_two(2, 3, 3, 5), Ok(8));
        assert_eq!(crypto::crt_two(1, 3, 4, 7), Ok(4));
    }

    #[test]
    fn euler_and_fermat_match_expected_values() {
        assert_eq!(crypto::euler_phi(10), Ok(4));
        assert_eq!(crypto::euler_phi(7), Ok(6));
        assert_eq!(crypto::fermat_check(3, 7), Ok(1));
    }

    #[test]
    fn sieve_and_primes_match_expected_values() {
        assert_eq!(crypto::is_prime(17), Ok(1));
        assert_eq!(crypto::is_prime(10), Ok(0));
        assert_eq!(crypto::prime_count(30), Ok(10));
        assert_eq!(crypto::nth_prime(30), Ok(113));
    }

    #[test]
    fn rsa_matches_expected_values() {
        assert_eq!(crypto::rsa_phi(61, 53), Ok(3120));
        assert_eq!(crypto::rsa_ed_check(7, 1783, 3120), Ok(1));
        assert_eq!(crypto::rsa_encrypt(42, 7, 3233), Ok(240));
        assert_eq!(crypto::rsa_decrypt(240, 1783, 3233), Ok(42));
    }

    #[test]
    fn verify_runs_probe_linked_cases() {
        let cases = vec![VerifyCase {
            expression: "modInverse(a, n) == 4".to_string(),
            function: "modInverse".to_string(),
            args: vec![
                VerifyArg::Probe("a".to_string()),
                VerifyArg::Probe("n".to_string()),
            ],
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
