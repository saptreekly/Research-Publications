# RSA Key Generation


RSA is built entirely on number theory that was developed over centuries, long before anyone imagined it would be used in cryptography.


## The Setup


Key generation in RSA has four steps:

1. Pick two large primes $p$ and $q$. Compute $n = pq$.
2. Compute $\phi(n) = (p-1)(q-1)$
3. Pick a public exponent $e$ such that $\gcd(e, \phi(n)) = 1$.
4. Compute the private exponent $d = e^{-1} \pmod{\phi{n}}$.

Your public key is $(e, n)$. Your private key is $(d, n)$. That's it.


## Step 1: Choose $p$, $q$, and $n$

In practice, $p$ and $q$ are randomly generated primes of equal bit length.

Typically they are $1024$ bits each, giving a $2048$-bit modulus.

I'll use small primes here so that the arithmetic is a bit more tangible.


## Step 2: Compute $\phi(n)$

$\phi(n)$ counts how many integers in $1$ to $n$ are coprime to $n$. For $n = pq$, the formula is:

$$\phi(pq) = (p-1)(q-1)$$

>See the Fermat and Euler notebook in Number Theory for the derivation.


## Step 3: Choose the Public Exponent $e$

$e$ must satisfy $\gcd(e, \phi(n)) = 1$. In real RSA, $e = 65537$ is almost universally used. It's prime, coprime to virtually any $\phi(n)$, and has a sparse binary representation that makes modular exponentiation fast.

For our stripped-down version, we can find the first valid $e$ automatically:


## Step 4: Compute the Private Exponent $d$

$d$ is the modular inverse of $e \text{ mod } \phi(n)$. It satisfies:

$$ed \equiv 1 \pmod{\phi(n)}$$

>This is computed using the Extended Euclidean Algorithm. See the Modular Inverses notebook in Number Theory for the derivation.


## The Keys


$n$ appears in both keys since it's the modulus for all the arithmetic and is public. What's secret is $d$.

An attacker who knows $n$ and $e$ would need to factor $n$ back into $p$ and $q$ to compute $\phi(n)$ and recover $d$. For a $2048$-bit $n$, this is infeasible with any known algorithm.


## Putting it all together


Here's a diagram showing the entire RSA key generation process with our example values:


## With Cryptographic-Scale Primes

The same code works, just use larger primes.


Encryption and decryption are covered in the next notebook.
