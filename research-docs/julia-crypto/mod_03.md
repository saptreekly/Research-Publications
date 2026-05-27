# Chinese Remainder Theorem


The Chinese Remainder Theorem (CRT) is one of the oldest proofs in number theory. It appears in Chinese mathematical texts from the 3rd century.


The question it answers is: *Given a system of congruences with coprime moduli, can we always find a solution?*


The answer is yes. And not only does a solution exist, it's unique modulo the product of the moduli.


## The Idea


Suppose you know what some unknown number $x$ satisfied:

$$x \equiv 2 \pmod{3}$$
$$x \equiv 3 \pmod{5}$$


We're not given $x$ directly, only these two remainders. Can you recover $x$?


We can do a naive search for it:


Notice how the solutions repeat with a regular period. Let's visualize this:


We get $x = 8$ and $x = 23$ and the gap between them is $15 = 3 \times 5$.

This is what the CRT predicts. Solutions repeat with period equal to the product of the moduli.


So the complete solution is $x \equiv 8 \pmod{15}$.


## Why Coprime Moduli?


Great question.

The moduli must be coprime for a solution to be guaranteed. If they share a factor, for example:

$$x \equiv 1 \pmod{2}$$
$$x \equiv 2 \pmod{4}$$

the system becomes inconsistent where no solution exists at all.

The first condition requires $x$ to be odd. The second requires $x$ to be even.

These are mutually exclusive.


We can verify that no solution exists:


## The General Statement


Given a system of $k$ congruences:

$$x \equiv a_1 \pmod{n_1}$$
$$x \equiv a_2 \pmod{n_2}$$
$$\vdots$$
$$x \equiv a_k \pmod{n_k}$$

where $n_1, n_2, \ldots, n_k$ are pairwise coprime, there exists a unique solution modulo $N = n_1 \cdot n_2 \cdots n_k$.


## Constructing the Solution


Like almost everything in cryptography, finding the solution by brute force works for small numbers but not at cryptographic scale. The CRT construction gives us a direct formula.

Let $N = n_1 \cdot n_2 \cdots n_k$. For each $i$, define:

$$N_i = \frac{N}{n_i}$$

$$M_i = N_i^{-1} \pmod{n_i}$$

Then the solution is:

$$x = \left(\sum_{i=1}^{k} a_i \cdot N_i \cdot M_i \right) \pmod{N}$$

Each term $a_i \cdot N_i \cdot M_i$ contributes exactly $a_i$ mod $n_i$ and $0$ mod all other moduli.

The sum picks up all the remainders simultaneously.


## Implementation


We can verify:


## Three-Congruence Example


CRT works for any number of congruences. Let's try three:

$$x \equiv 1 \pmod{3}$$
$$x \equiv 4 \pmod{7}$$
$$x \equiv 6 \pmod{11}$$


## Why This Matters for Cryptography


CRT shows up in two important places in RSA:

1. **Håstad's broadcast attack**: if the same message is encrypted to three recipients using $e = 3$ with no padding, an attacker has three ciphertexts $c_1, c_2, c_3$ satisfying:

$$m^3 \equiv c_1 \pmod{n_1}$$
$$m^3 \equiv c_2 \pmod{n_2}$$
$$m^3 \equiv c_3 \pmod{n_3}$$

CRT recovers $m^3$ directly, and since $m < n_i$ for all $i$, taking the integer cube root gives $m$. No factoring required.

2. **RSA-CRT decryption**: a standard optimisation in real RSA implementations that uses CRT to speed up decryption by a factor of ~4.

The Håstad attack is covered in the next notebook.
