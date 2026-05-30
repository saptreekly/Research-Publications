::: brief id=objectives title=Learning Objectives
> **LEARNING OBJECTIVES**
> * Implement the Sieve of Eratosthenes.
> * Understand $O(n \log \log n)$ prime generation.
> * Compare with Primes.jl in the next module.
:::

::: brief id=theory title=Theory: Sieve of Eratosthenes
Mark multiples of each prime starting at $i^2$. Unmarked integers up to $n$ are prime. The algorithm is one of the most efficient ways to enumerate all primes up to a limit.
:::

::: probe id=params
limit: 30, min=10, max=100
:::

::: starter id=sieve_impl lang=julia
```julia
function sieve(n)
    # TODO: Sieve of Eratosthenes — return a list of primes up to n
end

function is_prime(n)
    # TODO: Return 1 if n is prime, 0 otherwise (use your sieve)
end

function prime_count(n)
    # TODO: Return how many primes are <= n
end
```
:::

::: blueprint id=sieve_impl lang=julia
```julia
function sieve(n)
    is_prime = fill(true, n + 1)
    is_prime[1] = false

    for i in 2:n
        if is_prime[i] && i * i <= n
            for j in i*i:i:n
                is_prime[j] = false
            end
        end
    end

    return [i for i in 1:n if is_prime[i]]
end

function is_prime(n)
    n >= 2 && n in sieve(n)
end

println(sieve(30))
println(is_prime(17), is_prime(10))
```
:::

::: verify id=exercise_05
- is_prime(17) == 1
- is_prime(10) == 0
- is_prime(2) == 1
- prime_count(30) == 10
- prime_count(limit) == 10
:::

::: brief id=exercise title=Exercise 05
> **EXERCISE 05**
> * Implement the sieve and an `is_prime` helper.
> * Verify the count of primes up to 30 is 10.
:::
