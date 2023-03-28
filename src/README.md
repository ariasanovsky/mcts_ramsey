# Technical Details

- [Dynamic clique counting](#dynamic-clique-counting)
- [Action matrix](#action-matrix)
- [Data structures](#data-structures)
    - [Primary data structures](#primary-data-structures)
    - [Search maps](#search-maps)


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
$\argmax$ | action queue | `ActionMatrix` | `PriorityQueue<Action, i32>`

### Search maps

Mathematical function | struct | key struct | value struct
---|---|---|---
$\Gamma$ | `GraphMap` | `ColoredGraph` | `GraphData ~ (u32, ActionMap)`
$n(G, \cdot)$ | `ActionMap` | `Action` | `u32`
