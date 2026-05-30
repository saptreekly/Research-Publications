::: brief id=objectives title=Learning Objectives
> **LEARNING OBJECTIVES**
> * Use Primes.jl for primality testing and enumeration.
> * Find the $n$th prime with `prime(n)`.
> * Factor integers with `factor`.
:::

::: brief id=theory title=Theory: Primes.jl
RSA depends on the asymmetry between multiplying large primes and factoring their product. Primes.jl provides production-quality prime utilities used throughout cryptographic Julia code.
:::

::: probe id=params
n: 30, min=1, max=100
:::

::: starter id=primes_demo lang=julia
```julia
function nth_prime(n)
    # TODO: Return the nth prime (1-indexed). Hint: `prime(n)` is available as a builtin
end

function isprime_check(n)
    # TODO: Return 1 if n is prime, 0 otherwise
end
```
:::

::: blueprint id=primes_demo lang=julia
```julia
using Primes

println(isprime(7), isprime(10))
println(primes(30))
println(prime(30))  # 113
println(factor(561))  # 3 * 11 * 17
```
:::

::: verify id=exercise_06
- nth_prime(1) == 2
- nth_prime(6) == 13
- nth_prime(10) == 29
- nth_prime(30) == 113
- isprime_check(104729) == 1
:::

::: brief id=exercise title=Exercise 06
> **EXERCISE 06**
> * Explore Primes.jl functions in the blueprint.
> * Verify the $n$th prime mapping for small indices.
:::
