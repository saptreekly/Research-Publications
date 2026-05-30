::: brief id=objectives title=Learning Objectives
> **LEARNING OBJECTIVES**
> * Understand when a modular inverse exists ($\gcd(a,m)=1$).
> * Implement the Extended Euclidean Algorithm.
> * Normalize negative inverse coefficients into $[0, m)$.
:::

::: brief id=theory title=Theory: Modular Inverses
In $\mathbb{Z}/m\mathbb{Z}$, the inverse of $a$ is an integer $x$ such that $ax \equiv 1 \pmod m$.

An inverse exists **if and only if** $\gcd(a,m)=1$. When $\gcd(a,m)\neq 1$, no inverse exists (e.g. $2 \pmod 4$).

The Extended Euclidean Algorithm finds integers $u,v$ with $au+bv=\gcd(a,b)$. When $\gcd(a,m)=1$, reducing $au+bv=1 \pmod m$ gives $au \equiv 1 \pmod m$, so $u$ is the inverse.

Visualizations from the original notebook (clock face mod 7, gcd heatmap) are described in the curriculum theory section.
:::

::: probe id=params
a: 3, min=1, max=20
n: 11, min=2, max=50
:::

::: starter id=mod_inverse lang=julia
```julia
function extended_gcd(a, b)
    # TODO: Base case — when b == 0, return (a, 1, 0)
    # TODO: Recursive step using extended_gcd(b, a % b)
end

function modInverse(a, n)
    # TODO: Call extended_gcd(a, n); if gcd != 1, call error("Modular inverse does not exist")
    # TODO: Normalize the coefficient into [0, n)
end
```
:::

::: blueprint id=mod_inverse lang=julia
```julia
function extended_gcd(a, b)
    if b == 0
        return (a, 1, 0)
    end
    g, x1, y1 = extended_gcd(b, a % b)
    return (g, y1, x1 - (a ÷ b) * y1)
end

function modInverse(a, n)
    g, x, _ = extended_gcd(a, n)
    if g != 1
        error("Modular inverse does not exist")
    else
        return (x % n + n) % n
    end
end

println(modInverse(3, 11))
println(invmod(7, 10))  # Julia built-in: 3
```
:::

::: verify id=exercise_01
- modInverse(3, 11) == 4
- modInverse(5, 12) == 5
- modInverse(7, 10) == 3
- modInverse(2, 4) throws
- modInverse(a, n) == 4
:::

::: brief id=exercise title=Exercise 01
> **EXERCISE 01**
> * Implement `modInverse` using the Extended Euclidean Algorithm.
> * Compare with Julia's `invmod(a, m)` and `gcdx(a, m)`.
> * Adjust probe parameters and verify with the test cases.
:::
