# Fermat's Little Theorem and Euler's Theorem


In the modular inverses section, I established that $a^{-1} \pmod{m}$ exists **if and only if** $\gcd(a,m) = 1$. We also saw that `gcdx` gives us the inverse directly, if it exists.


The deeper question beneath all of this is: *why* does that structure exist at all? What is it about modular arithmetic that makes inverses behave this way?


## Fermat's Little Theorem


**Fermat's Little Theorem** is the first real answer to the question.

>If you've studied mathematics, you've likely heard of Fermat's *Last* Theorem. These are not the same.


It works like this:


Pick a prime number $p$ and any $a$ that is not divisible by it. That is, $a$ is not a multiple of $p$. Then, compute $a^{p-1} \pmod{p}$ for every valid $a$:


>`powermod(a, k, m)` computes $a^k \pmod{m}$ efficienty using repeated squaring. I'll cover what this actually is in the modular exponentiation notebook, but for now, think of it as a fast way to raise a number to a very large power and take the remainder.


Every result is $1$. Let's try a different prime:


Every result is 1! Let's visualize this pattern across different primes:


It doesn't take genius mathematician to recognize that that result too is all just $1$s.


This pattern holds for every prime, which is exactly what **Fermat's Little Theorem** states:

$$a^{p-1} \equiv 1 \pmod{p}$$

for any prime $p$ and any $a$ not divisible by $p$.


### The Inverse Corollary


It was later found that we can rewrite $a^{p-1}$ by splitting the exponent:

$$a^{p-1} = a \cdot a^{p-2}$$


This works because $1 + (p-2) = p-1$. Substituting into Fermat:

$$a \cdot a^{p-2} \equiv 1 \pmod{p}$$


This is exactly the form $a \cdot x \equiv 1 \pmod{p}$ which means $a^{p-2}$ **is** the modular inverse of $a$ mod $p$.


Both match but there's a catch... this only works when $p$ is a prime number. If the modulus is a composite, Fermat's Little Theorem doesn't hold and this breaks down entirely.


## Euler's Theorem


Euler's theorem picks up where Fermat's doesn't work.


It's written as:

$$a^{\phi(b)} \equiv 1 \pmod{b}$$


$\phi(b)$ is a number that answers: "How many integers in the range $1$ to $b$ are coprime to $b$?". This is known as **Euler's Totient Function**.


It works for any arbitrary $b$ as long as it is greater than $0$ and $\gcd(a,b) = 1$.


When $b$ is a prime number, this is the exact same as Fermat's Little Theorem.

$\phi(p) = p - 1$, therefore $a^{\phi (p)}=a^{p-1}$


We can verify Euler's Theorem the same way we verified Fermat's:


Verify $a^{\phi(b)} \equiv 1 \pmod{b}$ for all $a$ coprime to $b$:


Every $a$ coprime to $10$ gives $a^{\phi(10)} \equiv 1 \pmod{10}$.


Which numbers are coprime to n? Here's a visual showing φ(n) as filled circles:


Notice that values $2, 4, 5, 6,$ and $8$ are skipped since they share a factor with $10$.


With a prime modulus:


When $b$ is prime, every number from $1$ to $b-1$ is coprime to $b$.

And $\phi(7) = 6 = 7 - 1$, which is exactly Fermat's exponent.


This will become relevant to RSA encryption as RSA's modulus is $n = pq$ where $p$ and $q$ are two large primes, which is composite. Fermat breaks down here, but Euler doesn't.

$\phi(n) = (p-1)(q-1)$ as I will discuss, is what makes RSA work.
