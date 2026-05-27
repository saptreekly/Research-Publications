# Modular Inverses


The key question here is: "*what does it mean for a number for have a multiplicative inverse?*"


Every nonzero number $a$ has an inverse $a^{-1}$ such that $a \cdot a^{-1} = 1$.


For example, the inverse of $7$ is $7^{-1}$ or $\frac{1}{7}$.


In modular arithmetic, fractions don't exist since we only care about integers. So we need a better way to think about inverses.


Instead of asking "what is $a^{-1}$," we ask: *is there an integer $x$ such that $a \cdot x \equiv 1 \pmod{m}$?*


Modular arithmetic wraps around like a clock. Here's what it looks like for mod 7:


>*Quick note:* If you're new to cryptography or number theory $\equiv$ means "congruent to". Though I prefer to think of it as "these things are the same for some condition". For example, $7 \equiv 2 \pmod{5}$ means $7$ and $2$ have the same remainder when divided by $5$.


This doesn't always have an answer. As an example, we can try finding a solution to $2x \equiv 1 \pmod{4}$ for $x = 1, 2, 3$.


None of them work.


But $7x \equiv 1 \pmod{10}$ does have a solution:


$x = 3$ is a solution for $7x \equiv 1 \pmod{10}$.


The reason why we have a solution here and not in the previous example comes down to one condition: a modular inverse $a^{-1} (\text{mod } m)$ exists **if and only if** $\gcd(a,m) = 1$.


>$\gcd$ is the **Greatest Common Divisor** between two (or more) numbers.


We say that $a$ and $b$ are **coprime** (or **relatively prime**) if their $\gcd$ is $1$. This means they share **no** common factors except for $1$.


**This is the condition for a modular inverse to exist.**


Going back to our two examples:

- $\gcd(2, 4) = 2 \neq 1$ - No inverse exists for $2 \pmod{4}$. $2$ and $4$ are **not** coprime.
- $\gcd(7, 10) = 1$ - An inverse exists for $7 \pmod{10}$. $7$ and $10$ are coprime.


Julia has an operator built in for finding the gcd between two or more numbers: `gcd(a,b)`





But simply knowing an inverse *exists* is one thing. Finding it is another.


There are many different ways we can calculate the inverse. Here I'll focus on the **Extended Euclidean Algorithm**, though several others exists.


What's useful about the Euclidean algorithm is that helps us find $\gcd(a,b)$ easily but it also finds two integers $u$ and $v$, such that:

$$
au + bv = \gcd(a,b)
$$


When $a$ and $b$ are **coprime** $(\gcd(a,b) = 1)$, this becomes:

$$au + bv = 1$$


Reducing both sides $\text{mod } b$, the $bv$ term term disappears (as it's a multiple of $b$) leaving:

$$au \equiv 1 \pmod{b}$$


This means that $u$ **is** the modular inverse of $a \text{ mod } b$.


>I won't go into how the algorithm works here, but Numberphile has a great explainer video [here](https://www.youtube.com/watch?v=6Y3jHHE_hbA&t) if you're curious.


Believe it or not, Julia also has an operator baked into the language to calculate this: `gcdx(a,b)`.


`gcdx(7, 10)` returns $1$, $3$, and $-2$, where $1$ is the $\gcd(7, 10)$, $3$ is the inverse $(u)$, and $-2$ is our $v$.


We can verify this:


Julia also has a clean inverse modulo operator `invmod(a, m)` which is just a wrapper around the Euclidean algorithm:


>If you're reading down to here and are understanding it well, check out the section on RSA where all of this is practically relevant.


**A Note on Negative $u$**

Sometimes `gcdx` returns a negative $u$. Since we're working mod $m$, we can always add $m$ to bring it into the range $[0, m)$. The inverse is only defined up to multiples of $m$ anyway.


This still works:


Both $-2$ and $5$ are valid inverses of $3 \text{ mod } 7$. They're the same number in modular arithmetic.

In $\mathbb{Z}_7$, adding or subtracting $7$ doesn't change what a number *is*. The point of normalizing to a positive integer is simply because it's cleaner and easier to work with. That's it.

In $\mathbb{Z}_7, -6 \equiv 1$ since $-6 + 7 = 1$.

>$\mathbb{Z}_7$ is the set of all integers mod $7$. That is, $\{0, 1, 2, 3, 4, 5, 6\}$. Every integer lives somewhere on this set. If you're just starting out in mathematical cryptography, you can read up on *primitive roots*, but I'll be saving that for another section later on. It looks scary, but I promise that it becomes quite intuitive soon.
