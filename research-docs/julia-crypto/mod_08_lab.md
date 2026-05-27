::: brief id=objectives title=Learning Objectives
> **LEARNING OBJECTIVES**
> * Encrypt with $c \equiv m^e \pmod n$.
> * Decrypt with $m \equiv c^d \pmod n$.
> * Verify end-to-end RSA on small keys.
:::

::: brief id=theory title=Theory: RSA Encryption
Both encryption and decryption are a single modular exponentiation. The message $m$ must satisfy $m < n$. This educational implementation uses deterministic encryption without OAEP padding and is not suitable for production.
:::

::: probe id=params
message: 42, min=1, max=100
e: 7, min=3, max=20
d: 1783, min=100, max=5000
n: 3233, min=100, max=10000
:::

::: blueprint id=rsa_crypto lang=julia
```julia
p, q = 61, 53
n = p * q
e = 7
d = invmod(e, (p - 1) * (q - 1))

m = 42
c = powermod(m, e, n)
m_decrypted = powermod(c, d, n)

println("Message:    $m")
println("Ciphertext: $c")
println("Decrypted:  $m_decrypted")
```
:::

::: verify id=exercise_08
- rsa_encrypt(42, 7, 3233) == 240
- rsa_decrypt(240, 1783, 3233) == 42
- rsa_encrypt(message, e, n) == 240
- rsa_decrypt(240, d, n) == 42
:::

::: brief id=exercise title=Exercise 08
> **EXERCISE 08**
> * Encrypt message 42 with the $(e=7, n=3233)$ public key.
> * Decrypt the ciphertext with the private key.
> * Confirm the round-trip matches the original message.
:::
