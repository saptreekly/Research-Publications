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
            result: Some("-".to_string()),
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
        ("modInverse", 2) => mod_inverse(args[0], args[1]),
        ("powermod", 3) => powermod(args[0], args[1], args[2]),
        ("crt_two", 4) => crt_two(args[0], args[1], args[2], args[3]),
        ("euler_phi", 1) => euler_phi(args[0]),
        ("fermat_check", 2) => fermat_check(args[0], args[1]),
        ("is_prime", 1) => is_prime(args[0]),
        ("prime_count", 1) => prime_count(args[0]),
        ("nth_prime", 1) => nth_prime(args[0]),
        ("isprime_check", 1) => isprime_check(args[0]),
        ("rsa_phi", 2) => rsa_phi(args[0], args[1]),
        ("rsa_ed_check", 3) => rsa_ed_check(args[0], args[1], args[2]),
        ("rsa_encrypt", 3) => rsa_encrypt(args[0], args[1], args[2]),
        ("rsa_decrypt", 3) => rsa_decrypt(args[0], args[1], args[2]),
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

fn powermod(base: i64, exp: i64, modulus: i64) -> Result<i64, String> {
    if modulus <= 0 {
        return Err("Modulus must be positive.".to_string());
    }
    if exp < 0 {
        return Err("Exponent must be non-negative.".to_string());
    }
    let mut result = 1_i64;
    let mut b = base % modulus;
    let mut e = exp;
    while e > 0 {
        if e % 2 == 1 {
            result = (result * b) % modulus;
        }
        b = (b * b) % modulus;
        e /= 2;
    }
    Ok(result)
}

fn crt_two(r1: i64, m1: i64, r2: i64, m2: i64) -> Result<i64, String> {
    if m1 <= 0 || m2 <= 0 {
        return Err("Moduli must be positive.".to_string());
    }
    let n = m1 * m2;
    let n1 = n / m1;
    let n2 = n / m2;
    let inv1 = mod_inverse(n1, m1)?;
    let inv2 = mod_inverse(n2, m2)?;
    Ok((r1 * n1 * inv1 + r2 * n2 * inv2) % n)
}

fn euler_phi(n: i64) -> Result<i64, String> {
    if n <= 1 {
        return Ok(0);
    }
    let mut count = 0;
    for i in 1..n {
        if gcd(i, n) == 1 {
            count += 1;
        }
    }
    Ok(count)
}

fn fermat_check(a: i64, p: i64) -> Result<i64, String> {
    if p <= 1 {
        return Err("Prime must be greater than 1.".to_string());
    }
    powermod(a, p - 1, p)
}

fn sieve_primes(limit: i64) -> Result<Vec<i64>, String> {
    if limit < 0 {
        return Err("Limit must be non-negative.".to_string());
    }
    let n = limit as usize;
    let mut is_prime = vec![true; n + 1];
    if n >= 1 {
        is_prime[1] = false;
    }
    for i in 2..=n {
        if is_prime[i] && (i as i64) * (i as i64) <= limit {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    Ok((2..=limit).filter(|&i| is_prime[i as usize]).collect())
}

fn is_prime(n: i64) -> Result<i64, String> {
    if n < 2 {
        return Ok(0);
    }
    let primes = sieve_primes(n)?;
    Ok(if primes.contains(&n) { 1 } else { 0 })
}

fn prime_count(n: i64) -> Result<i64, String> {
    Ok(sieve_primes(n)?.len() as i64)
}

fn nth_prime(n: i64) -> Result<i64, String> {
    if n <= 0 {
        return Err("Index must be positive.".to_string());
    }
    let mut count = 0_i64;
    let mut candidate = 2_i64;
    loop {
        if is_prime(candidate)? == 1 {
            count += 1;
            if count == n {
                return Ok(candidate);
            }
        }
        candidate += 1;
        if candidate > 1_000_000 {
            return Err("Search limit exceeded.".to_string());
        }
    }
}

fn isprime_check(n: i64) -> Result<i64, String> {
    is_prime(n)
}

fn rsa_phi(p: i64, q: i64) -> Result<i64, String> {
    if p <= 1 || q <= 1 {
        return Err("Primes must be greater than 1.".to_string());
    }
    Ok((p - 1) * (q - 1))
}

fn rsa_ed_check(e: i64, d: i64, phi: i64) -> Result<i64, String> {
    if phi <= 0 {
        return Err("Phi must be positive.".to_string());
    }
    Ok(if (e * d) % phi == 1 { 1 } else { 0 })
}

fn rsa_encrypt(message: i64, e: i64, n: i64) -> Result<i64, String> {
    if message >= n {
        return Err("Message must be less than n.".to_string());
    }
    powermod(message, e, n)
}

fn rsa_decrypt(ciphertext: i64, d: i64, n: i64) -> Result<i64, String> {
    powermod(ciphertext, d, n)
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
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
    fn powermod_matches_expected_values() {
        assert_eq!(powermod(2, 10, 13), Ok(10));
        assert_eq!(powermod(3, 13, 97), Ok(31));
        assert_eq!(powermod(2, 1000, 13), Ok(3));
    }

    #[test]
    fn crt_two_matches_expected_values() {
        assert_eq!(crt_two(2, 3, 3, 5), Ok(8));
        assert_eq!(crt_two(1, 3, 4, 7), Ok(4));
    }

    #[test]
    fn euler_and_fermat_match_expected_values() {
        assert_eq!(euler_phi(10), Ok(4));
        assert_eq!(euler_phi(7), Ok(6));
        assert_eq!(fermat_check(3, 7), Ok(1));
    }

    #[test]
    fn sieve_and_primes_match_expected_values() {
        assert_eq!(is_prime(17), Ok(1));
        assert_eq!(is_prime(10), Ok(0));
        assert_eq!(prime_count(30), Ok(10));
        assert_eq!(nth_prime(30), Ok(113));
    }

    #[test]
    fn rsa_matches_expected_values() {
        assert_eq!(rsa_phi(61, 53), Ok(3120));
        assert_eq!(rsa_ed_check(7, 1783, 3120), Ok(1));
        assert_eq!(rsa_encrypt(42, 7, 3233), Ok(240));
        assert_eq!(rsa_decrypt(240, 1783, 3233), Ok(42));
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
