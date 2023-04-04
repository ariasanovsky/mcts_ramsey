# Technical Details

- [Dynamic clique counting](#dynamic-clique-counting)
- [Action matrix](#action-matrix)
- [Data structures](#data-structures)
  - [Primary data structures](#primary-data-structures)
  - [Search maps](#search-maps)
- [Improvements](#improvements)
  - [Replace $n(G, a)$ wwith $n(G')$](#replace-with)
  - [Better hashes](#better-hashes)
  - [Better graph guessing](#better-graph-guessing)

## Dynamic clique counting

To update $\kappa_G$ following action $a = c'|uv$ with $c$ the current color of $uv$, we do not change the values of $\kappa_G[c][uv]$ and $\kappa_G[c'][uv]$.
The recoloring affects $\kappa_G[c][uw]$ for every $vw\in E(G[c])$, and similarly for $c'$ replacing $c$, and $u,v$ interchanged.
It also affects $\kappa_G[c][wx]$ for every distinct $w,x\in [N]\setminus\{u,v\}$ such $w,x$ are both neighbors of $u$ and of $v$ in $G[c]$ (and similarly for $c'$ replacing $c$).

## Action matrix

The counts $\kappa_G[c][uv]$ are stored in a $C\times \binom{N}{2}$ array of integers and a hash-map based priority queue ranks actions by $\delta(G, a)$.
With $c$ the color of $uv$ in $G$, the action $a = c'|uv$ does not a affect the values of $\kappa_G[c][uv]$ and $\kappa_G[c'][uv]$. However, cliques of $G[c]$ ($G[c']$) containing $u,v$ are removed (added), so the row $\kappa_G[c][\cdot]$ ($\kappa_G[c'][\cdot])$ must be updated with care.
Each affected entry $(c, wx)$ ($(c', wx)$) affects $\delta(G, d|wx)$ for each valid action of the form $d|wx$.

## Data structures

### Primary data structures

Mathematical object | name | struct or parent struct | example implementation
---|---|---|--|
$G$ | (colored) graph | `ColoredGraph` | `[[u64; N]; C]`
$a = c\vert uv$ | (recoloring) action | `Action` | `(usize, usize)`
$\nabla G$ | discrete gradient vector | `ActionMatrix` | `[[i32; E]; C]`
$\text{arg max}$ | action queue | `ActionMatrix` | `PriorityQueue<Action, i32>`

### Search maps

Mathematical function | struct | key struct | value struct
---|---|---|---
$\Gamma$ | `GraphMap` | `ColoredGraph` | `GraphData ~ (u32, ActionMap)`
$n(G, \cdot)$ | `ActionMap` | `Action` | `u32`

## Improvements

We consider the following potential `todo!()` items.

### Replace $n(G, a)$ with $n(G')$

As motivation, we wish to

1. reduce memory consumption, and
2. decrease "commmutative redundancy".

For 1., the search centers on a large hash map $\Gamma$ that records for each visited graph $G$ the value $n(G)$, and a hash map $n(G, \cdot)$ keyed by the actions valid from $G$.
It may be possible to eliminate the need for the second hash map.

For 2., suppose $a$ and $b$ are actions valid from $G$ which correspond to recoloring different edges (i.e., $a = c|uv$ and $b = d|wx$ where $uv\neq wx$).
Also, let $aG$ be the graph formed by taking $a$ from $G$.
Define $bG$, $abG$, and $baG$ similarly.
Since $a,b$ recolor different edges, $abG = baG$.
If $q(G, a)$ is slightly larger than $q(G, b)$, then the agent will take actions which increment $n(G, a)$ followed by $n(aG, b)$, but $n(G, b)$ and $n(bG, a)$ will be slower to increment.
When exploration biases the agent towards $b$ over $a$, the agent will eagerly visit $baG$ only to encounter a node that is already visited many times.
This fundamentally defeats the purpose of exploration.

Recall that from graph $G$, action $a$ is chosen so as to maximize

$$
\mu(G, \cdot) = \delta(G, \cdot) + \nu(G, \cdot)
$$

over the set of actions valid from $G$.

The choice of noise function

$$
\nu(G, a) = \dfrac{C\cdot \sqrt{n(G)}}{1+n(G,a)}
$$

may be improvable.
Currently, the algorithm finds $\text{argmax}_a \mu(G, a)$ as follows.
For convenience,

- let $A(G)$ be the actions valid from $G$,
- let $\hat{A}(G)$ be the actions taken from $G$ during the search, and
- let $\check{A}(G) := A(G)\setminus \hat{A}(G)$.

1. let $\hat{a}$ be an argmax of $\mu(G, \cdot)$ over $\hat{A}(G)$
    - Note that $\hat{A}(G)$ is the set of keys of the hash map $n(G, \cdot)$
2. let $\check{a}$ be an argmax of $\delta(G, \cdot)$ over $\hat{A}(G)$
    - Since $n(G, \cdot) = 0$ on $\check{A}(G)$, this is equivalent to maximizing $\mu(G, \cdot)$
3. let $a^*$ be the argmax of $\mu(G, \cdot)$ over $\{\hat{a}, \check{a}\}$ and return $a^*$

Early in the search, $\hat{A}(G)$ is small, so step 1. is not expensive at first.
Recall that by implementaiton with a hash map priority queue, $A(G)$ is dynamically sorted by $\delta(G, \cdot)$.
As long as $\hat{A}(G)$ is small, few dequeues and requeues are required in step 2.

As an alternative to $\nu$, we define for each $a\in A(G)$, with $G'$ formed by taking $a$ from $G$, the quantities

$$
\nu'(G, a) := \dfrac{C\cdot \sqrt{n(G)}}{1+n(G')} \text{ and } \mu'(G, a) := \delta(G, a) + \nu'(G, a).
$$

To maximize $\mu'(G, \cdot)$ over $A(G)$, we may proceed as follows.

- iterate over $A(G)$ in $\delta(G, \cdot)$-nonincreasing order with a running value $a^*$ approaching the argmax of $\mu'(G,\cdot)$
  - halt the search if $\delta(G, a)\leq \mu'(G, a^*) - C\sqrt{n(G)}$
  - for each $a$, let $a^*$ be the argmax of $\mu(G, \cdot)$ over $\{a^*, a\}$

The stopping condition is due to the fact that $\nu'(G, a)\in \left[0, C\sqrt{n(G)}\right]$.
We cannot improve on $a^*$ once $\delta(G, a)$ is sufficiently small.

Property | $\nu$ | $\nu'$
---|---|---
memory consumption | requires storing a map $n(G, \cdot)$ keyed by actions $a$ taken from $G$ during the search | relies only on the map $n(\cdot)$ keyed by visited graphs
complexity of noise | requires looking up the value $n(G, a)$ for each visited action $a$ | requires constructing $G'$ and looking up $n(G')$

I estimate that the number of dequeue operations is quite comparable between the two.

### Better hashes

For ordering actions by $q$, the index `(c, pos)` corresponding to the action of recoloring the edge at position `pos` with color `c` is a key to the priority queue's hash map.
Since the dimensions are known to be $C\times \binom{N}{2}$, we can instead replace the default hash map with a perfect hash.
The details are outside of my current understanding, but if the cost is cheap, I would eagerly do it!
It is tempting to reimplement the priority queue in-place over the action matrix, but I consider this too expensive to test and optimize.

Additionally, the default hash algorithm in rust is fast, but prioritizes security.
Other hash functions, such as [HashBrown](https://lib.rs/crates/hashbrown) may perform better.
We have 3 hash maps to benchmark: `GraphMap`, `ActionMap`, and `ActionMatrix`.

### Parallel searching

With multiple agents, we can:

1. Visit more graphs
2. Test multiple exploration rates simultaneously

As a cost, this requires:

1. Resolving the memory problem first
2. Refactoring the mutation steps to avoid concurrency errors

### Better graph guessing

Currently, the first root graph $G$ is selected as a uniformly random $C$-edge-colored graph.
When $S$ is nonuniform, this choice is far from optimal.
Better would be to select $G\sim\mathbb{G}(N, p^*)$ where $p^*$ minimizes the expected cost

$$
\left\langle
1,
\mathbb{E}_{G\sim \mathbb{G}(N, p)}\left[
\kappa_S(G)
\right]\right\rangle
=
\sum_c \mathbb{E}_{G\sim\mathbb{G}(N, p_c)}\left[ \kappa_{S[c]}(G) \right]
=
\sum_c\binom{N}{S[c]}\cdot p_c^{\binom{S[c]}{2}}
$$

over all $C$-dimensional probability vectors $p$.
As a second optimization, we may also use the vector

$$
\mathbb{E}_{G\sim \mathbb{G}(N, p^*)}\left[
\kappa_S(G)
\right]
$$

to define a nonuniform weighting for the cost function (i.e., so that a red $K_3$ does not cost the same as a blue $K_9$).
