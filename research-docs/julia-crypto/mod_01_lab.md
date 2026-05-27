::: brief id=objectives title=Learning Objectives
> **LEARNING OBJECTIVES**
> * Understand the mechanics of modular exponentiation.
> * Implement the Extended Euclidean Algorithm for inverse calculation.
:::

::: brief id=theory title=Theory: The Euclidean Ring
To perform cryptographic operations, we rely on the ring $\mathbb{Z}/n\mathbb{Z}$. The core primitive is finding the modular multiplicative inverse.

Given $a$ and $n$, find $x$ such that $ax \equiv 1 \pmod n$.
:::

::: probe id=params
a: 3, min=1, max=20
n: 11, min=2, max=50
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
```
:::

::: verify id=exercise_01
- modInverse(3, 11) == 4
- modInverse(5, 12) == 5
- modInverse(2, 4) throws
- modInverse(a, n) == 4
:::

::: brief id=exercise title=Exercise 01
> **EXERCISE 01**
> * Implement `modInverse` in Julia using the blueprint above.
> * Adjust probe parameters and verify your understanding with the test cases.
> * Expected: `modInverse(3, 11)` returns `4`.
:::
