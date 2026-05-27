# RSA Encryption and Decryption


With the keys from the key generation notebook, encryption and decryption is surprisingly simple. Both are just a single modular exponentiation.


## Setup

Reusing the keys from the key generation notebook:


## Encryption

Given a message $m$ (as an integer, where $m \lt n$), encryption is:

$$c = m^e \pmod{n}$$

The message is raised to the public exponent and reduced mod $n$.

The output is the ciphertext $c$.


$42$ becomes $240$. Without knowing $d$, recovering $m$ from $c$ requires solving the **discrete logarithm problem**, which is effectively impossible for large $n$.


## Decryption

Given ciphertext $c$, decryption is:

$$m = c^d \pmod{n}$$

The private exponent $d$ undoes what $e$ did.


RSA encryption is a bijection (permutation) on the message space. Every plaintext maps to exactly one ciphertext:


## Why Does This Work?

This is essentially just Euler's Theorem.

Recall that $d$ is chosen so that $ed \equiv 1 \pmod{\phi(n)}$. This means that $ed = 1 + k\phi(n)$ for some integer $k$. So:

$$m^{ed} = m^{1 + k\phi(n)} = m \cdot (m^{\phi(n)})^k$$

By Euler's Theorem, $m^{\phi(n)} \equiv 1 \pmod{n}$ when $\gcd(m, n) = 1$. So:

$$m^{ed} \equiv m \cdot 1^k \equiv m \pmod{n}$$

Encrypting with $e$ then decrypting with $d$ gives back $m$. This is because $e$ and $d$ are modular inverses of each other in the exponent.


## Limitations

The implementation I've written here is intentionally limited. RSA in practice is much more complex in two important ways: Padding and Small Public Exponents.

Encrypting the same message twice with the same key always produces the same ciphertext. This is a very serious problem. An attacker can build a dictionary of known plaintexts and ciphertexts. Real RSA uses padding schemes like OAEP to randomize encryption.

Using a small $e$, like $7$ in this example, without padding opens the door to attacks. If the same message is encrypted to several recipients with the same small $e$, an attacker can recover the message using the **Chinese Remainder Theorem**. This is Håstad's broadcast attack which is covered in a separate notebook.
