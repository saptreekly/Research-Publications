# Modular Exponentiation


In other notebooks in this repo, `powermod(a, k, m)` shows up a lot. This notebook is to explain what it actually does and why it exists.


The naive approach to computing $a^k \pmod{m}$ is quite straightforward: raise $a$ to the power of $k$, then take the remainder.


This is more or less fine for small numbers. But with a cryptographic-scale exponent:


We get an integer overflow. $a^k$ outputs garbage.

We could fix this with `big()`:


That works, but it's hiding the problem that in RSA, $k$ is a $2048$-bit number. Computing $a^k$ naively at this scale would require approx. $2^{2048}$ multiplications.

>For context, the number of atoms in the observable universe is about $2^{270}$


Point is, we need a better approach for RSA.


## Repeated Squaring


The key insight is that exponentiation has a structure that can be exploited easily.

Instead of multiplying $a$ by itself $k$ times, we can repeatedly square:

$$a \rightarrow a^2 \rightarrow a^4 \rightarrow a^8 \rightarrow a^{16} \rightarrow \ldots$$


Each step is just one multiplication. Thus, to reach $a^{2048}$, we only need $11$ multiplications instead of $2047$ that the naive approach requires.


But $k$ won't aways be a power of $2$. The trick is to use binary representation of $k$.

For example, $13$ in binary is $1101$, which means:

$$13 = 8 + 4 + 1$$

So:

$$a^{13} = a^8 \cdot a^4 \cdot a^1$$


We can visualize this a bit better:


One thing to point out in the output from the cell above is how `base` double in exponent each step, and `result` only updates when the current bit is $1$.

The modular reduction happens at every step, which keeps numbers small throughout the whole algorithm

Most importantly, consider how we never needed `big()`. Because we reduce mod $m$ at every multiplication, the numbers never grow beyond $m^2$.


Here's a visual breakdown of how the algorithm processes each bit of k=13:


Although Julia does offer the `powermod()` method out of the box, a manual representation of the algorithm would look like this:


Let's test it against Julia's method:


They match! Let's use even larger numbers:


In RSA, encryption is $c = m^e \pmod{n}$ and decryption is $m = c^d \pmod{n}$, both of which are modular exponentiation operations where the exponents $e$ and $d$ are large, cryptographic-scale numbers.

Without repeated squaring, neither operation would be feasible and RSA would not be the encryption standard of today.

`powermod` is what makes RSA actually useful.
