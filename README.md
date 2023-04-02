# Tabular MCTS Ramsey search

## Setup

To set up, [install rust](https://www.rust-lang.org/tools/install), clone the repo, and execute `cargo run` in the project directory.

Inside `./plots`, you will find newly generaated images

<center>
    <img src="./r[3, 3]_5_0.svg" alt="R(3,3)>5" width="100"/>
    <img src="./r[3, 3]_5_1.svg" alt="R(3,3)>5" width="100"/>
</center>

which witness the lower bound $R(3,3) > 5$.
Your construction may not look the same, but it is isomorphic (identical up to vertex relabeling) to this one.

The output

```powershell
==== EPOCH ==== 1
score improved to 0 by
vertex 0: [1, 4] [2, 3]
vertex 1: [0, 2] [3, 4]
vertex 2: [1, 3] [0, 4]
vertex 3: [2, 4] [0, 1]
vertex 4: [0, 3] [1, 2]
0; 10; 110; 0110;
["Dhc", "DUW"]
1 minimum... ==== DONE ====
```

describes the construction found by the program.
First, vertices are paired with their neighborhoods in each color.
Second, the edges are enumerated in colex order with their colors.
Third, the colored graphs are shortened in [g6 format](http://users.cecs.anu.edu.au/~bdm/data/formats.txt).
Verbosity is determined by $N$.

## Custom Ramsey problems

```powershell
$Env:N=16 ; $Env:S=3,3,3 ; $Env:EPOCHS=100 ; $Env:EPISODES=10000 ; $Env:EXPLORE=5.5 ; cargo run --release
```

finds a witness to the bound $R(3,3,3) > 16$, using the release build.
Environment variables are managed by `build.rs` and are known at **compile-time**.


**Be mindful of memory consumption** when the program runs for too long.
Each (colored) graph visited in the search is stored as a $N\times C$-dimensional array of `u8`, `u16`, ..., or `u128` depending on $N$.
Additionally, each action taken is also stored in memory with an incrementing visit count.

## States and Actions

The agent seeks to minimize the cost equal to number of colored cliques corresponding to the Ramsey problem.
For example, when $S = (3, 4)$, the cost equals the number of red $K_3$'s plus the number of blue $K_4$'s in the host colored graph.
For actions, the agent selects an edge $uv$ and decides on a new color for $uv$.
Altogether, the search space is $\binom{N}{2}$-dimensional with $C$ possible values in each coordinate.
At each state, $(C-1)\times \binom{N}{2}$ actions are viable.

## Dynamic programming

[Clique counting is hard](https://en.wikipedia.org/wiki/Clique_problem).
In order to save resources, we employ dynamic programming.
Each colored graph is associated with a $C\times \binom{N}{2}$-dimensional vector which records, for each color $c$ and each edge $uv$, the number of $S[c]$-cliques in color $c$ that contain both vertices $u,v$, **ignoring the color of $uv$**.
Formally, this is a function $\kappa_G:[C]\times \binom{[N]}{2}\to\mathbb{N}$ where 

$$
\kappa_G[c][uv] =
\text{number of copies of }K_{S[c]}\text{ in } G[c]+uv.
$$

When $uv$ has color $c$, the action $c'\vert uv$ (recolor $uv$ with color $c'$) changes the cost (clique count) by

$$
\nabla G[c'][uv] := \kappa_G[c'][uv] - \kappa_G[c][uv].
$$

We also employ a hash map priority queue to optimally order actions (`src/README.md`).

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
The exploration constant $C$ is chosen *ad hoc*.
Here, $n(G)$ is the number of visits to $G$ during the search, and $n(G, a)$ is the number of times $a$ is taken from $G$, both dependent on search time.

## Results

We summarize our results compared to [known](https://www.combinatorics.org/files/Surveys/ds1/ds1v15-2017.pdf) [Ramsey numbers](https://en.wikipedia.org/wiki/Ramsey's_theorem).

We select a high exploration rate $\approx 4.5$ by default.

### $R(3, k)$

$R(3,k)$ | constructed lower bound | first-success elapsed time | construction (`g6` format)
---|---|---|---
$R(3,3) = 6$  | tight | 595.337ms | ```["ElSg", "EQjO"]```
$R(3,4) = 9$  | tight | 521.408ms | ```["GLSMHG", "Gqjpus"]```
$R(3,5) = 14$ | tight | 580.478ms | ```["Lo_gQkacHcJCPE", "LN^VlR\\ZuZszmx"]```
$R(3,6) = 18$ | tight | 601.449ms | ```["P?X?wROb@@Ck?u[C[?SiO_qO", "P~e~Fkn[}}zR~Hbzb~jTn^Lk"]```
$R(3,7) = 23$ | tight | 818.718ms | ```["U?@Q?uyOH_WH_aH@aAHWcBaGQCTI_?OqJ`?HQ?h_", "U~}l~HDnu^fu^\\u}\\|ufZ{\\vlzit^~nLs]~ul~UW"]```
$R(3,8) = 28$ | tight | 3.658s | ```["Z?@wq@AWeOK?GQ?GaCu_A?@QXsDE@SEIAEAGKY?KOb?_jg?I?CH@EIOaCPHO", "Z~}FL}\|fXnr~vl~v\\zH^\|~}leJyx}jxt\|x\|vrd~rn[~^SV~t~zu}xtn\\zmug"]```
$R(3,9) = 36$ | $\geq 33$ | 825.932ms | ```["_SB?A_EH`@L?HoCKB?O{O_p?c@gGXA?`bAH??SKG_?L??YO?GaUC`@CXO?oSBGAWOKIB@GWGDCQQAAPR?FA?", "_j{~\|^xu]}q~uNzr{~nBn^M~Z}Vve\|~][\|u~~jrv^~q~~dn~v\\hz]}zen~Nj{v\|fnrt{}vfvyzll\|\|mk~w\|{"]```
| | $\geq 34$ | 191.455s | ```["`_?GC?A?toR???DOTOYOe??HCS`?GBBK@CWIUW_cg_UCo`A?Ir__GoGCkb?@OaO?]o??pAI?GskCC?_B?hGKAG^@?", "`^~vz~\|~INk~~~ynindnX~~uzj]~v{{r}zfthf^ZV^hzN]\|~tK^^vNvzR[~}n\\n~`N~~M\|t~vJRzz~^{~Uvr\|v_}~"]```
| | $\geq 35$ | no results

### $R(4, k)$

$R(4,k)$ | constructed lower bound | first-success elapsed time | construction (`g6` format)
---|---|---|---
$R(4,4) = 18$ | tight | $283.924$ ms | ```["PcP_zojhJi\\omTkjXFG{tt{?", "PZm^CNSUsTaNPiRSewvBIIB{"]```
$R(4,5) = 25$ | tight | 9.517s (lucky) | ```["WroZCHIHgEjy\\Mo^EKKN?iC]OIR[UoX`yx[FjwgCkRIbXf_", "WKNczutuVxSDapN_xrro~Tz`ntkbhNe]DEbwSFVzRkt[eW^"]```
$R(4,6) \in [36,40]$ | $\geq 33$ | 28.975s | ```["_g@\|{ui_DSua`AChBDPxsp`h[RbAExADaXCV@sWPhdDHjoAS[bPwc`]YMIX_CxSsIHQlpGH@J~AXIGK`e_sK", "_V}ABHT^yjH\\]\|zU{ymEJM]Ubk[\|xE\|y\\ezg}JfmUYyuSN\|jb[mFZ]`dpte^zEjJtulQMvu}s?\|etvr]X^Jo"]```
| | $\geq 34$ | no results

### $R(5, k)$

$R(5,k)$ | constructed lower bound | first-success elapsed time | construction (`g6` format)
---|---|---|---
$R(5,5) \in [43,48]$ | $\geq 35$ | 32.017s | ```["aT`gWGl^JK{WUQkEJW]flSwRX}@MJuyMJKki[lJBnDW~q_LijxkQeeCBNbEgNqmMZtYsIhbhp\\mSvU?t]q^ARHmkSoJK{\\_", "ai]VfvQ_srBfhlRxsf`WQjFke@}psHDpsrRTbQs{Oyf?L^qTSERlXXz{o[xVoLPpcIdJtU[UMaPjGh~I`L_\|kuPRjNsrBaW"]```
| | $\geq 36$ | 65.205s | ```["bWPLpU[]ZLXGqhlRAZ\|nqwohwXLwFWhzXO[ktRM~@\|cmOyc`[~OiZQYoxQH[\\mm?\\GQrRZHG`y~C[\|SHp\\TgzmfE`Dd_~^DxGdjR?", "bfmqMhb`cqevLUQk\|cAOLFNUFeqFwfUCenbRIkp?}AZPnDZ]b?nTcldNElubaPP~avlKkcuv]D?zbAjuMaiVCPWx]yY^?_yEvYSk_"]```
| | $\geq 37$ | no results

## Multicolor Ramsey numbers

$R(S)$ | constructed lower bound | first-success elapsed time | construction (`g6` format)
---|---|---|---
$R(3,3,3) = 17$ | tight | 7.852s | ```["O?va?gU[@POZk_OQdWC@b", "O[G?qE@bUKJcAIHgIAXiC", "Ob?\\LPg?gac?PTeDOdaSW"]```
$R(3,3,4) = 30$ | $\geq 26$ | 120.286s | ```["X@qF??^@Q_o?gIHQG?hOG_?BP?OmD_sd?dKHEdq?@i[Kg`GDom?", "XMD?cY?AH[Cc?soD?q?_oA@km`E??Y@OPGaOGWKCODaA@KC?D?t", "XoGwZd_{cBJZV@EgvLUNF\\}O?]hPyDIImQPepA@zmO@pUQryIPI"]```
| | $\geq 27$ | no results
$R(3,3,3,3) \in [51, 62] $ | no results
