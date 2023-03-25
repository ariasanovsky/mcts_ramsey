# Technical Details

- [Crude Monte Carlo search](#crude-monte-carlo-search)
- [Dynamic clique counting](#dynamic-clique-counting)
- [Action matrix](#action-matrix)
- [Data structures](#data-structures)
    - [Primary data structures](#primary-data-structures)
    - [Search maps](#search-maps)
- [Primer on Ramsey Theory](#primer-on-ramsey-theory)


## Crude Monte Carlo Search

We begin with a simple MCTS search.
A (colored) graph is a $C$-edge-colored complete graph on some number $N$ of vertices.
The `action` $a = c'\vert uv$ denotes the replacement of colored edge $(c, uv)$ with the colored edge $(c', uv)$ where $c' \neq c$.
Our cost function equals the total number of "bad" cliques with respect to the given Ramsey problem.
For details, see the `Primer on Ramsey Theory`.

Let $\delta(G, a)$ be the *decrease* in cost after action $a$.
For the efficient computation of $\delta(G, a)$, see `Dynamic clique counting`.
While searching, we log the number $n(G)$ of visits to graph $G$, and the number $n(G, a)$ of times taking $a$ from $G$.
Both are implicitly functions of the search time.
Our `noise` term $\nu(G, a)$, also a function of search time, will be

$$
\nu(G, a) := \dfrac{C\cdot \sqrt{n(G)}}{1+n(G,a)}
$$

where $C$ is a constant chosen *ad hoc* to improve the search.

Throughout the search, we take action $a$ from $G$ if it is the argmax of

$$
\mu(G, a) := \delta(G, a) + \nu(G, a).
$$

For efficient determination of $\argmax_a \mu(G, a)$, see `Action matrix`.

## Dynamic clique counting

`todo!("write, merge, move")`

## Action matrix

`todo!("write, merge, move?")`

## Data structures

### Primary data structures

Mathematical object | name | struct or parent struct | example implementation
---|---|---|--|
$G$ | (colored) graph | `ColoredGraph` | `[[u64; N]; C]`
$a = c\vert uv$ | (recoloring) action | `Action` | `(usize, usize)`
$\nabla(G)$ | discrete gradient vector | `ActionMatrix` | `[[i32; E]; C]`
$\argmax$ | action queue | `ActionMatrix` | `PriorityQueue<Action, i32>`

### Search maps

Mathematical function | struct | key struct | value struct
---|---|---|---
$\Gamma$ | `GraphMap` | `ColoredGraph` | `GraphData ~ (u32, ActionMap)`
$n(G, \cdot)$ | `ActionMap` | `Action` | `u32`

## Primer on Ramsey Theory

`todo!("probably to be moved")`