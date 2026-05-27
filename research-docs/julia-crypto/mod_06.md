# Primes.jl Demo


Prime numbers are what make cryptography actually work. RSA depends entirely on the fact that multiplying two large primes together is easy, but factoring the result back into those primes is effectively impossible.


[Primes.jl](https://juliamath.github.io/Primes.jl/stable/api/) is Julia's dedicated prime number library. This notebook is a tour of its most useful functions.


## `isprime`

The most basic question: is this number prime?


Simple enough for small numbers. But for cryptographic use, primes need to be hundreds of digits long.

Testing primality naively (checking every divisor up to $\sqrt{n}$ ) is practically impossible that this scale.


`isprime` in Julia uses a combination of trial division for small factors and the Miller-Rabin primality test for large numbers, the latter I'll cover in depth in a later notebook.


## `primes` and `prime`


Though they both look and sound similar, these two functions answer two different questions:

- `primes(n)` gives all primes **up to** $n$.
- `prime(n)` gives the $n\text{th}$ prime.


Let's look at the gaps between consecutive primes to see the pattern in their distribution:


This is equivalent to running the **Sieve of Eratosthenes** up to $30$, a topic which deserves it's own dedicated notebook.


For anyone who's manually written the Sieve of Eratosthenes with an upper bound estimate knows how much cleaner that is.

`prime(n)` handles all of that internally.


## `factor`


`factor(n)` returns the complete prime factorization of $n$ as a `Factorization` object (essentially a dictionary mapping each prime factor to its exponent).


As touched on earlier, the reason factoring matters in cryptography is that it's a fundamentally difficult problem.

Given $n = pq$ in RSA where $p$ and $q$ are both large prime numbers, there is no known efficient algorithm for recovering $p$ and $q$ from $n$ alone. This is essentially what makes RSA secure.


## `nextprime` and `prevprime`


These return the nearest prime above or below a given value:


Note that these take a **value**, not an index.

`nextprime(100)` means "the first prime greater than $100$, not "the $100\text{th}$ prime". Compare:


In practice, `nextprime` is how you'd generate primes for RSA key generation.

Essentially: pick a random large number and find the nearest prime:


That's a cryptographic-scale prime generated in one line.

In RSA, two primes not unlike the one above become $p$ and $q$.


## Some Bonus Useful Functions


A few more functions worth knowing:


### `totient`

This computes Euler's totient function $\phi(n)$ directly (see notebook on Fermat's and Euler's theorems for explanation of both).

*"How many integers in $1:n$ are coprime to $n$?"*


### `eachfactor`

This iterates over prime, exponent pairs.


### `radical`

This returns the product of distinct prime factors.


### `ismersenneprime`

Checks if $n$ is of the form $2^p - 1$.


> Note: `ismersenneprime` throws an error if the input is not a valid Mersenne number of the form $2^p - 1$. It doesn't just return `false`.
