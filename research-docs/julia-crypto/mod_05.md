# Sieve of Eratosthenes


In the `Primes.jl` demo, `primes(n)` handed us all primes up to $n$ in one line.

This notebook is about what's actually happening under the hood and why I think it's the most elegant algorithm in all of mathematics.


The problem is quite straightforward: find all prime numbers up to some limit $n$.


## The Naive Approach

The obvious approach is to test each number individually. You might call it "brute forcing".

For every number $k$ up to $n$, check if anything divides it evenly.

Like most algorithms in cryptography, it works for small $n$, but for large $n$ it's infeasible.

For each number you're computing up to $\sqrt{k}$ division checks, which is a time complexity of roughly $O(n\sqrt{n})$.


There's a much better way.


## The Sieve

The key insight is to flip the question. Instead of asking *"is this prime?"* for each number, assume everything is prime and cross out the composites.


The algorithm is:

1. Create a list of booleans, all set to `true`, indexed from $1$ to $n$.
2. Mark index $1$ as `false`, since $1$ is neither prime nor composite.
3. For each number $i$ from $2$ to $\sqrt{n}$: if $i$ is still marked `true`, cross out all multiples of $i$ starting from $i^2$.
4. Everything still marked `true` is prime.


Two things worth noting before I show you the code behind it:

- We only need to sieve up to $\sqrt{n}$ because any composite number $k$ must have a factor $\leq \sqrt{k}$.
- We start crossing out at $i^2$ rather than $2i$ because all smaller multiples of $i$ have already been crossed out by earlier primes.


Let's verify this against `Primes.jl`:


Let's visualize the sieve algorithm on a grid to see which numbers get crossed out:


## Why start at $i^2$?

When we reach $i$ in the outer loop, every multiple of $i$ is below $i^2$ and can be written as $i \times k$ where $k \lt i$.

Since $k \lt i$, we already processed $k$ in a previous iteration.

Starting at $i^2$ skips all that redundant work.


This is what gives the Sieve its time complexity $O(n\log \log n)$, which is nearly linear.


Here's how the actual prime count π(n) compares to the prime number theorem's approximation:


## Project Euler Problem 7

What is the $10001 \text{st}$ prime?


## Project Euler Problem 10

Sum of all primes below $2$ million:


## Limitations

Although the Sieve is fast, it's quite memory hungry. It requires a boolean array of size $n$. For $n = 600{,}851{,}475{,}143$ (Project Euler problem $3$ ), that's hundreds of gigabytes. In practice, segmented sieves or probabilistic primality tests like Miller-Rabin are used instead.

For cryptographic purposes, `nextprime` from `Primes.jl` is the practical tool since it uses Miller-Rabin internally and works at any scale.


The sieve's real value isn't as a production tool. It's one of the clearest examples of algorithmic thinking in mathematics. A 2,200-year-old algorithm that still runs in nearly linear time.
