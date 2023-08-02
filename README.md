# Multiplication in big fields with fast lookup tables

All ideas credited to [Axiom](https://github.com/axiom-crypto).

Here we suggest an implementation of multiplication in a large field, under the assumption that lookup tables work very fast.

## Setup

Suppose we want to emulate multiplication modulo $p$ over a native modulus $n$. In other words, we want to find witnesses $q$ and $r$, such that

$ab = qp + r$

over $\mathbb Z$ and produce a proof only using constraints that are defined modulo $n$.

### How large things are

In practice, $p$ and $n$ are $254$-bit primes and $p$ is slightly larger than $n$. 

Lookup tables can have at most $2^{28}$ rows. In this project we assume that the number of lookup tables is unlimited.

## The classic: [Aztec](https://hackmd.io/@arielg/B13JoihA8)

The original implementation by Aztec uses Chinese Remainder Theorem. We find a large number $M$, coprime to $n$, such that $M\cdot n > p^2$, and then perform the checks:

1. Find $q$, $r$, such that $ab = qp + r$;
2. Range check $0 \le ab < M\cdot n$;
3. Range check $0 \le qp + r < M\cdot n$;
4. Native operation $ab - qp - r = 0 \mod n$;
5. $ab - qp - r = 0 \mod M$.

The final check is the tricky one. In the original implementation by Aztec, this check was handled as follows: $M$ was chosen to be $2^t$ for $t$ large enough, and then the additions and multiplications were implemented by accessing bits of our variables.

In reality, all steps are slightly more complicated, since the numbers are too large for the lookup tables.


# Strategy

### Lookup tables to look things up 

Here is a way to perform non-native multiplication using a lookup table: we can find $ab \mod m$ using the  table with $m^2$ rows, whose rows are triples $(a, b, ab \mod m)$.

Now in order to check that $ab = r \mod m$, we need to check that $(a, b, r)$ is in the lookup table (if $0\le r < m$).

For this to work, we would need the number of rows $m^2$ to be at most $2^{28}$, so $m < 2^{14}$

## Preparation

First, we chose $M = m_1\cdot \ldots \cdot m_t$ so that
1. All $m_i$ are relatively prime;
2. $m_i < 2^{14}$ for all $i$;
3. $Mn > p^2 + p$.
(I suspect, we want $t \ge 20$).

Now we want to pre-compute lookup tables $(a,\; b,\; ab \mod m_i)$ as above for all $i$, as well as $p_i = p \mod m_i.$

## High-level algorithm

1. Find $q$, $r$, such that $ab = qp + r$;
2. Range check $0 \le ab < M\cdot n$: this is done by checking $0\le a < p$ and $0 \le b < p$;
3. Range check $0 \le qp + r < M\cdot n$:  this is done by checking $0\le q < p$ and $0 \le r < p$;
4. Check that $a \cdot b =  q \cdot p + r \mod n$;
5. $a$, $b$, $q$, $r$ are converted into the CRT form: $a \mapsto (a_1, a_2, \ldots)$ where $a_i = a \mod m_i$;
6. For each modulus $m_i$, check that $a_i \cdot b_i =  q_i \cdot p_i + r_i \mod m_i$ by using a lookup table.

## Partial reduction

This bit is inspired by [Aztec](https://hackmd.io/@arielg/B13JoihA8) implementation, as well as this [paper](https://eprint.iacr.org/2022/1470.pdf). 

Traditionally, computations with big integers use limbs: $a = \sum_{i= 0} a_i 2^{i\;\cdot \; limb_{-}bits}$.