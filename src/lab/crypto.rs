pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, x1, y1) = extended_gcd(b, a % b);
    (g, y1, x1 - (a / b) * y1)
}

pub fn mod_inverse(a: i64, n: i64) -> Result<i64, String> {
    if n <= 0 {
        return Err("Modulus must be positive.".to_string());
    }
    let (g, x, _) = extended_gcd(a, n);
    if g != 1 {
        return Err("Modular inverse does not exist.".to_string());
    }
    Ok(((x % n) + n) % n)
}

pub fn powermod(base: i64, exp: i64, modulus: i64) -> Result<i64, String> {
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

pub fn crt_two(r1: i64, m1: i64, r2: i64, m2: i64) -> Result<i64, String> {
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

pub fn euler_phi(n: i64) -> Result<i64, String> {
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

pub fn fermat_check(a: i64, p: i64) -> Result<i64, String> {
    if p <= 1 {
        return Err("Prime must be greater than 1.".to_string());
    }
    powermod(a, p - 1, p)
}

pub fn sieve_primes(limit: i64) -> Result<Vec<i64>, String> {
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

pub fn is_prime(n: i64) -> Result<i64, String> {
    if n < 2 {
        return Ok(0);
    }
    if n == 2 {
        return Ok(1);
    }
    if n % 2 == 0 {
        return Ok(0);
    }

    let mut divisor = 3;
    while divisor * divisor <= n {
        if n % divisor == 0 {
            return Ok(0);
        }
        divisor += 2;
    }

    Ok(1)
}

pub fn prime_count(n: i64) -> Result<i64, String> {
    Ok(sieve_primes(n)?.len() as i64)
}

pub fn nth_prime(n: i64) -> Result<i64, String> {
    if n <= 0 {
        return Err("Index must be positive.".to_string());
    }

    let mut limit = 32_i64;
    loop {
        let primes = sieve_primes(limit)?;
        if primes.len() >= n as usize {
            return Ok(primes[n as usize - 1]);
        }
        limit = limit.saturating_mul(2);
        if limit > 1_000_000 {
            return Err("Search limit exceeded.".to_string());
        }
    }
}

pub fn isprime_check(n: i64) -> Result<i64, String> {
    is_prime(n)
}

pub fn rsa_phi(p: i64, q: i64) -> Result<i64, String> {
    if p <= 1 || q <= 1 {
        return Err("Primes must be greater than 1.".to_string());
    }
    Ok((p - 1) * (q - 1))
}

pub fn rsa_ed_check(e: i64, d: i64, phi: i64) -> Result<i64, String> {
    if phi <= 0 {
        return Err("Phi must be positive.".to_string());
    }
    Ok(if (e * d) % phi == 1 { 1 } else { 0 })
}

pub fn rsa_encrypt(message: i64, e: i64, n: i64) -> Result<i64, String> {
    if message >= n {
        return Err("Message must be less than n.".to_string());
    }
    powermod(message, e, n)
}

pub fn rsa_decrypt(ciphertext: i64, d: i64, n: i64) -> Result<i64, String> {
    powermod(ciphertext, d, n)
}

pub fn factor_display(n: i64) -> Result<String, String> {
    if n <= 1 {
        return Ok(n.to_string());
    }
    let mut value = n;
    let mut factors = Vec::new();
    let mut divisor = 2;
    while divisor * divisor <= value {
        while value % divisor == 0 {
            factors.push(divisor);
            value /= divisor;
        }
        divisor += 1;
    }
    if value > 1 {
        factors.push(value);
    }
    Ok(factors
        .iter()
        .map(i64::to_string)
        .collect::<Vec<_>>()
        .join(" * "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factor_display_matches_example() {
        assert_eq!(factor_display(561), Ok("3 * 11 * 17".to_string()));
    }
}
