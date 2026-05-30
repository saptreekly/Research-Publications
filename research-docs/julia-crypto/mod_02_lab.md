::: brief id=objectives title=Learning Objectives
> **LEARNING OBJECTIVES**
> * Understand why naive $a^k \pmod m$ overflows for large $k$.
> * Implement square-and-multiply (binary exponentiation).
> * Relate `powermod` to RSA encryption steps.
:::

::: brief id=theory title=Theory: Modular Exponentiation
Computing $a^k \pmod m$ by raising $a$ to the $k$ first fails for large exponents. Intermediate values explode before the final `% m`.

Square-and-multiply processes bits of $k$: square the base each step, multiply into the result when the current bit is 1. Julia's `powermod(a, k, m)` uses this internally.
:::

::: probe id=params
base: 3, min=2, max=20
exp: 13, min=1, max=100
modulus: 97, min=2, max=200
:::

::: starter id=powermod_impl lang=julia
```julia
function powermod(a, k, m)
    # TODO: Square-and-multiply — keep every intermediate value in [0, m)
    # Hint: result = 1, base = a % m, loop while k > 0
end
```
:::

::: blueprint id=powermod_impl lang=julia
```julia
function manual_powermod(a, k, m)
    result = 1
    base   = a % m
    e      = k

    while e > 0
        if isodd(e)
            result = (result * base) % m
        end
        base = (base * base) % m
        e    = div(e, 2)
    end

    return result
end

a, k, m = 3, 13, 97
println(manual_powermod(a, k, m))
println(powermod(a, k, m))
```
:::

::: verify id=exercise_02
- powermod(2, 10, 13) == 10
- powermod(3, 13, 97) == 31
- powermod(2, 1000, 13) == 3
- powermod(base, exp, modulus) == 31
:::

::: brief id=exercise title=Exercise 02
> **EXERCISE 02**
> * Implement square-and-multiply in `manual_powermod`.
> * Verify against Julia's `powermod` for the probe values.
:::
