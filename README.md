# Tabular MCTS Ramsey search

## Setup

To set up, [install rust](https://www.rust-lang.org/tools/install), clone the repo, and execute `cargo run` in the project directory.

This (hopefully) constructs a lower bound for the [Ramsey problem](https://en.wikipedia.org/wiki/Ramsey's_theorem) $R(3,3)$.
It should look something like this:

<center><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/9/98/RamseyTheory_K5_no_mono_K3.svg/1280px-RamseyTheory_K5_no_mono_K3.svg.png" alt="R(3,3)>5" width="300"/></center>
<center>R(3,3) > 5</center>

The output
```
==== EPOCH ==== 1
score improved to 0 by
vertex 0: [2, 4] [1, 3]
vertex 1: [3, 4] [0, 2]
vertex 2: [0, 3] [1, 4]
vertex 3: [1, 2] [0, 4]
vertex 4: [0, 1] [2, 3]
1; 01; 100; 0011;
["DRo", "DkK"]
1 minimum... R[3, 3] > 5
```
describes the construction found by the program.
First, vertices are paired with their neighborhoods in each color.
Second, the edges are enumerated in colex order with their colors.
Third, the colored graphs are shortened in [g6 format](http://users.cecs.anu.edu.au/~bdm/data/formats.txt).
Verbosity is determined by $N$.

## Custom Ramsey problems

```powershell
$Env:N=8 ; $Env:S=3,4 ; $Env:EPOCHS=100 ; $Env:EPISODES=10000 ; $Env:EXPLORE=0.3 ; cargo run --release
```

finds a witness to the bound $R(3,4) > 8$, using the release build.
Be mindful of memory consumption when the program runs for too long.
Each (colored) graph visited in the search is stored as a $N\times C$-dimensional array of `u8`, `u16`, ..., or `u128` depending on $N$.
Additionally, each action taken is also stored in memory.

## States and Actions

The agent seeks to minimize the cost equal to number of colored cliques corresponding to the Ramsey problem.
For example, when $S = (3, 4)$, the cost equals the number of red $K_3$'s plus the number of blue $K_4$'s in the host colored graph.
For actions, the agent selects an edge $uv$ and decides on a new color for $uv$.
Altogether, the search space is $\binom{N}{2}$-dimensional with $C$ possible values in each coordinate.
At each state, $(C-1)\times \binom{N}{2}$ actions are viable.

## Dynamic programming

[Clique counting is hard](https://en.wikipedia.org/wiki/Clique_problem).
In order to save resources, we employ dynamic programming.
Each colored graph is associated with a $C\times \binom{N}{2}$-dimensional vector which records, for each color $c$ and each edge $uv$, the number of $S[c]$-cliques in color $c$ that contain both vertices $u,v$.
Formally, this is a function $\kappa_G:[C]\times \binom{[N]}{2}\to\mathbb{N}$ where 

$$
\kappa_G[c][uv] = \#
\text{ copies of }K_{S[c]}\text{ in } G[c]+uv.
$$

When $uv$ has color $c$, the action $c'\vert uv$ (recolor $uv$ with color $c'$) changes the cost (clique count) by

$$
\nabla G[c'][uv] := \kappa_G[c'][uv] - \kappa_G[c][uv].
$$

## Monte Carlo Search

From state $G$, we select action $a = c'\vert uv$ so as to maximize

$$
\mu(G, a) := \delta(G, a) + \nu(G, a)
$$

where $\delta(G, a) :=  - \nabla G[c'][uv]$ is the immediate improvement in the score function and

$$
\nu(G, a) := \dfrac{C\cdot \sqrt{n(G)}}{1+n(G,a)}
$$

is time-dependent noise which biases the agent towards exploration.
The constant $C$ is chosen *ad hoc*.
Here, $n(G)$ is the number of visits to $G$ during the search, and $n(G, a)$ is the number of times $a$ is taken from $G$.

## Results
