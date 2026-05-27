## 01.1 / MODULAR FOUNDATIONS

> **LEARNING OBJECTIVES**
> * Understand the mechanics of modular exponentiation.
> * Implement the Extended Euclidean Algorithm for inverse calculation.

### Theory: The Euclidean Ring
To perform cryptographic operations, we rely on the ring $\mathbb{Z}/n\mathbb{Z}$. The core primitive is finding the modular multiplicative inverse. 

Given $a$ and $n$, find $x$ such that $ax \equiv 1 \pmod n$. 

### Implementation: The Julia Blueprint
```julia
function modInverse(a, n)
    g, x, y = extended_gcd(a, n)
    if g != 1
        error("Modular inverse does not exist")
    else
        return (x % n + n) % n
    end
end
```

> **EXERCISE 01**
> * Implement `modInverse` in Julia.
> * Test it against $a=3, n=11$ (Expected: 4).
