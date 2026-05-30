::: brief id=objectives title=Learning Objectives
> **LEARNING OBJECTIVES**
> * State Fermat's Little Theorem and Euler's Theorem.
> * Compute Euler's totient $\phi(n)$.
> * Use $a^{\phi(n)-1}$ as a modular inverse when $\gcd(a,n)=1$.
:::

::: brief id=theory title=Theory: Fermat and Euler
**Fermat's Little Theorem:** If $p$ is prime and $\gcd(a,p)=1$, then $a^{p-1} \equiv 1 \pmod p$.

**Euler's Theorem:** If $\gcd(a,n)=1$, then $a^{\phi(n)} \equiv 1 \pmod n$, where $\phi(n)$ counts integers in $[1,n)$ coprime to $n$.

For prime $p$, $\phi(p)=p-1$, recovering Fermat.
:::

::: probe id=params
a: 3, min=1, max=20
n: 10, min=2, max=50
:::

::: starter id=euler_impl lang=julia
```julia
function euler_phi(n)
    # TODO: Count integers in 1:n-1 that are coprime to n
end

function fermat_check(a, p)
    # TODO: Return a^(p-1) mod p using powermod(a, p - 1, p)
end
```
:::

::: blueprint id=euler_impl lang=julia
```julia
function euler_phi(n)
    count = 0
    for i in 1:n-1
        if gcd(i, n) == 1
            count += 1
        end
    end
    return count
end

function fermat_check(a, p)
    return powermod(a, p - 1, p)
end

println(euler_phi(10))       # 4
println(fermat_check(3, 7))  # 1
```
:::

::: verify id=exercise_04
- euler_phi(10) == 4
- euler_phi(7) == 6
- fermat_check(3, 7) == 1
- fermat_check(2, 13) == 1
- euler_phi(n) == 4
:::

::: brief id=exercise title=Exercise 04
> **EXERCISE 04**
> * Implement `euler_phi` by counting coprime integers.
> * Verify Fermat's Little Theorem for probe values when $\gcd(a,n)=1$.
:::
