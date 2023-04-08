include!(concat!(env!("OUT_DIR"), "/constants.rs"));

use bit_fiddler::{set, unset, is_set, mask};
use itertools::Itertools;
use rand::prelude::*;
use rand::distributions::WeightedIndex;
use bit_iter::BitIter;

pub type Color = usize;
pub type Vertex = usize;
pub type Edge = (Vertex, Vertex);

pub struct ColoredEdge {
    pub color: Color,
    pub edge: Edge
}
pub struct Recoloring<const N: usize> {
    pub old_color: Color,
    pub new_color: Color,
    pub edge: Edge
}

impl<const N: usize> Recoloring<N> {
    pub fn old_edge(&self) -> ColoredEdge { ColoredEdge { color: self.old_color, edge: self.edge } }
    pub fn new_edge(&self) -> ColoredEdge { ColoredEdge { color: self.new_color, edge: self.edge } }
}

pub const fn choose(n: usize, k: usize) -> Iyy {
    match (n, k) {
        (_, 0) => 1,
        (0, _) => 0,
        _ => choose(n-1, k) + choose(n-1, k-1)
    }
}

pub const fn choose_two(n: usize) -> usize {
    n*(n+1)/2-n
}

pub fn pos_to_edge<const N: usize>(pos: usize) -> Edge {
    let mut u = 0;
    let mut pos_u_n_minus_1 = N - 2;
    while pos_u_n_minus_1 < pos {
        pos_u_n_minus_1 += N - 2 - u;
        u += 1
    }
    let v = N - 1 - (pos_u_n_minus_1 - pos);
    (u, v)
}

pub fn edge_to_pos<const N: usize>((u, v): Edge) -> usize {
    if u < v { u*(2*N-1-u)/2 + (v-u-1) }
    else     { v*(2*N-1-v)/2 + (u-v-1) }
}

#[cfg(test)]
mod math_tests {

    use itertools::Itertools;

    use super::*;

    #[test]
    fn choose_two_consistent() {
        for n in 0..100 {
            assert_eq!(choose_two(n), choose(n, 2) as usize);
        }
    }

    const N: usize = 8;

    #[test]
    fn pos_to_edge_test() {
        for (i, (u, v)) in (0..N).tuple_combinations().enumerate() {
            assert_eq!(pos_to_edge::<N>(i), (u, v));
        }
    }

    #[test]
    fn edge_to_pos_test() {
        for (i, (u, v)) in (0..N).tuple_combinations().enumerate() {
            assert_eq!(edge_to_pos::<N>((u,v)), i);
            assert_eq!(edge_to_pos::<N>((v,u)), i);
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct ColoredGraph<const C: usize, const N: usize> {
    neighborhoods: [[Uxx; N]; C]
}

pub fn random_edge<const N: usize>(rng: &mut ThreadRng) -> Edge {
    let u = rng.gen_range(0..N);
    let v = rng.gen_range(0..N-1);
    if v < u { (v, u) } else { (u, v+1) }
}

impl<const C: usize, const N: usize> ColoredGraph<C, N> {
    pub fn score(&self) -> Iyy {
        (0..C)
        .map(|c| self.count_cliques(c, None, None))
        .sum()
    }
    
    pub fn count_cliques(&self, color: Color, s: Option<usize>, candidates: Option<Uxx>) -> Iyy {
        let s = s.unwrap_or(S[color]);
        if s == 0 { return 1 }
        
        let candidates = candidates.unwrap_or(mask!([0..N], Uxx));
        if s == 1 { return candidates.count_ones() as Iyy } 
        
        BitIter::from(candidates)
            .map(|u| candidates & self.bit_neighborhood(color, u) & mask!([u..N], Uxx))
            .map(|candidates| self.count_cliques(color, Some(s-1), Some(candidates)))
            .sum()
    }

    pub fn count_edge_cliques(&self, color: Color, (u, v): Edge) -> Iyy {
        let candidates = Some(
            self.common_neighborhood(color, u, v));
        self.count_cliques(color, Some(S[color]-2), candidates)
    }
    
    pub fn red() -> ColoredGraph<C, N> {
        let mut neighborhoods: [[Uxx; N]; C] = [[0; N]; C];
        for u in 0..N {
            let mut neighborhood = mask!([0..N], Uxx);
            unset!(in neighborhood, Uxx, u);
            neighborhoods[0][u] = neighborhood
        }
        ColoredGraph { neighborhoods }
    }

    pub fn uniformly_random(rng: &mut ThreadRng) -> ColoredGraph<C, N> {
        let mut neighborhoods: [[Uxx; N]; C] = [[0; N]; C];
        for (u, v) in (0..N).tuple_combinations() {
            let c = rng.gen_range(0..C);
            neighborhoods[c][u] = set!((neighborhoods[c][u]), Uxx, v);
            neighborhoods[c][v] = set!((neighborhoods[c][v]), Uxx, u);
        }
        
        ColoredGraph { neighborhoods }
    }

    pub fn random(rng: &mut ThreadRng, dist: &WeightedIndex<f64>) -> ColoredGraph<C, N> {
        let mut neighborhoods: [[Uxx; N]; C] = [[0; N]; C];
        for (u, v) in (0..N).tuple_combinations() {
            let c = rng.sample(dist);
            neighborhoods[c][u] = set!((neighborhoods[c][u]), Uxx, v);
            neighborhoods[c][v] = set!((neighborhoods[c][v]), Uxx, u);
        }
        
        ColoredGraph { neighborhoods }
    }

    fn add(&mut self, c: Color, edge: Edge) {
        let (u, v) = edge;
        self.neighborhoods[c][u] = set!((self.neighborhoods[c][u]), Uxx, v);
        self.neighborhoods[c][v] = set!((self.neighborhoods[c][v]), Uxx, u);
    }

    fn delete(&mut self, c: Color, edge: Edge) {
        let (u, v) = edge;
        self.neighborhoods[c][u] = unset!((self.neighborhoods[c][u]), Uxx, v);
        self.neighborhoods[c][v] = unset!((self.neighborhoods[c][v]), Uxx, u);
    }

    pub fn recolor(&mut self, recolor: Recoloring<N>) {
        self.delete(recolor.old_color, recolor.edge);
        self.add(recolor.new_color, recolor.edge);
    }

    pub fn has_edge(&self, colored_edge: ColoredEdge) -> bool {
        let ColoredEdge { color, edge } = colored_edge;
        let (u, v) = edge;
        
        is_set!((self.bit_neighborhood(color, u)), Uxx, v)
    }

    pub fn color(&self, (u, v): Edge) -> Option<Color> {
        self.neighborhoods
            .iter()
            .position(
                |&neighborhood| 
                is_set!((neighborhood[u]), Uxx, v))
    }

    pub fn bit_neighborhood(&self, color: Color, u: Vertex) -> Uxx {
        self.neighborhoods[color][u]
    }

    pub fn common_neighborhood(&self, color: Color, u: Vertex, v: Vertex) -> Uxx {
        self.bit_neighborhood(color, u) & 
        self.bit_neighborhood(color, v)
    }

    pub fn random_edge(&self, rng: &mut ThreadRng) -> ColoredEdge {
        let edge = random_edge::<N>(rng);
        let color = self.color(edge)
            .unwrap();
        ColoredEdge { color, edge }
    }

    pub fn random_recoloring(&self, rng: &mut ThreadRng) -> Recoloring<N> {
        let colored_edge = self.random_edge(rng);
        let new_color = rng.gen_range(0..C-1);
        let new_color = 
            if new_color < colored_edge.color { new_color }
            else {new_color + 1};
        Recoloring { 
            old_color: colored_edge.color,
            new_color,
            edge: colored_edge.edge 
        }
    }

    pub fn randomly_recolor(&mut self, rng: &mut ThreadRng) {
        self.recolor(self.random_recoloring(rng))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_red_cliques() {
        const N: usize = 8;
        const C: usize = 2;
        let red = ColoredGraph::<C, N>::red();
        assert_eq!(choose(N, S[0]),
            red.count_cliques(0, None, None));
        for c in 1..C {
            assert_eq!(0,
                red.count_cliques(c, None, None))
        }
    }
}

impl<const C: usize, const N: usize> From<[[Uxx; N]; C]> for ColoredGraph<C, N> {
    fn from(neighborhoods: [[Uxx; N]; C]) -> Self {
        ColoredGraph { neighborhoods }
    }
}