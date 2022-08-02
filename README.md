# ECFFT
[![CI Check](https://github.com/ashWhiteHat/ecfft/actions/workflows/test.yml/badge.svg)](https://github.com/ashWhiteHat/ecfft/actions/workflows/test.yml)  
[`ECFFT`](https://arxiv.org/pdf/2107.08473.pdf) implementation and benchmark optimized by [`rayon`](https://github.com/rayon-rs/rayon).

## Abstract
When performing the polynomials multiplication by traditional FFT, it's necessary to find the multiplicative group $G: |G| = n = 2^k$ satisfying $n\ |\ p - 1$.

$$
P(X)\ *\ Q(X)\ where\ P(X),\ Q(X)\ ∈ F_p[X]
$$

The ECFFT can remove the limitation of $n\ |\ p - 1$ that traditional FFT needs.

## Arithmetic
When using Cooley Tukey algorithm, the FFT structure is following.

$$
P(X) = P_0(ψ(X)) + XP_1(ψ(X))
$$

$P_0$ and $P_1$ are respectively even and odd degree half size polynomials of $P(X)$. Evaluate half size polynomials at $ψ(X)$ recursively and get $P(X)$ with $\mathcal{O}(n\log{}n)$.

## Precompute
### Common
Firstly, finding the $ψ$ and precomputing the domain of $P_0$ and $P_1$.
Classic FFT uses $ψ: X \rightarrow X^2$ so it's not necessary to find $ψ$.

### Classic FFT
1. Finding root of unity.
2. Calculating multiplicative group for each recursion whose group order is $2^k$ staring with power root of unity.

### ECFFT
1. Finding curve whose group order is $2^k$.
2. Finding isogeny which halves group size.
3. Calculating multiplicative group for each recursion halved by isogeny.

### FFTree
$P(X)$ degree is $2^k$ and, $P_0(X)$ and $P_1$ degree are $2^{k-1}$ because these  are respectively even and odd polynomials of $P(X)$. $X$ is the domain of polynomial and multiplicative group whose order is $2^k$. Through divide and conquer algorithm, the polynomial and multiplicative group are halved in each layer. It can be expressed as binary tree of depth $k$. Precomputing multiplicative group and composing binary tree can be done in advance. This binary tree is called FFTree and consists of domain of each layer.

※ The multiplicative group generated by root of unity can be thought as elements on unit circle so just doubling the $θ$(array index) and get the half size of multiplicative group so it's not necessary to be FFTree.

## Curve
ECFFT curve can be defined as following.

$$
y^2 = x^3 + x + 23665697887148517506426806798051226694671519983424102823343279587811911881026
$$

You can find curve, subgroup and isogenies with [utils](https://github.com/ashWhiteHat/ecfft_utils).
