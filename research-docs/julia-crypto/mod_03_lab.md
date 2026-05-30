::: brief id=objectives title=Learning Objectives
> **LEARNING OBJECTIVES**
> * Solve systems of congruences with pairwise coprime moduli.
> * Implement the Chinese Remainder Theorem construction.
> * Recognize when no solution exists (non-coprime moduli).
:::

::: brief id=theory title=Theory: Chinese Remainder Theorem
Given $x \equiv a_i \pmod{n_i}$ with pairwise coprime moduli, CRT constructs the unique solution modulo $N = \prod n_i$:

$$x = \sum a_i \cdot N_i \cdot \text{invmod}(N_i, n_i) \pmod N$$

where $N_i = N / n_i$.
:::

::: probe id=params
r1: 2, min=0, max=10
m1: 3, min=2, max=20
r2: 3, min=0, max=10
m2: 5, min=2, max=20
:::

::: starter id=crt_impl lang=julia
```julia
function crt_two(r1, m1, r2, m2)
    # TODO: N = m1 * m2, N1 = N ÷ m1, N2 = N ÷ m2
    # TODO: Combine residues with invmod(N1, m1) and invmod(N2, m2)
end
```
:::

::: blueprint id=crt_impl lang=julia
```julia
function crt_two(r1, m1, r2, m2)
    N = m1 * m2
    N1 = N ÷ m1
    N2 = N ÷ m2
    x = (r1 * N1 * invmod(N1, m1) + r2 * N2 * invmod(N2, m2)) % N
    return x
end

println(crt_two(2, 3, 3, 5))  # x = 8
println(8 % 3, 8 % 5)
```
:::

::: verify id=exercise_03
- crt_two(2, 3, 3, 5) == 8
- crt_two(1, 3, 4, 7) == 4
- crt_two(r1, m1, r2, m2) == 8
:::

::: brief id=exercise title=Exercise 03
> **EXERCISE 03**
> * Implement `crt_two` for two congruences.
> * Verify $x \equiv 2 \pmod 3$ and $x \equiv 3 \pmod 5$.
> * Extend to three moduli using the same pattern from the notebook.
:::
