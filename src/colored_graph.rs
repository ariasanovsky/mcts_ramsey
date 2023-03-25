pub use crate::r_3_3::*;

use bit_fiddler::{set, unset, is_set, mask};
use rand::prelude::*;
use bit_iter::BitIter;

pub type Color = usize;
pub type Vertex = usize;
pub type Edge = (Vertex, Vertex);

pub struct ColoredEdge {
    pub color: Color,
    pub edge: Edge
}
pub struct Recoloring {
    pub old_color: Color,
    pub new_color: Color,
    pub edge: Edge
}

impl Recoloring {
    pub fn old_edge(&self) -> ColoredEdge { ColoredEdge { color: self.old_color, edge: self.edge } }
    pub fn new_edge(&self) -> ColoredEdge { ColoredEdge { color: self.new_color, edge: self.edge } }
}

pub const fn choose(n: usize, k: usize) -> usize {
    match (n, k) {
        (_, 0) => 1,
        (0, _) => 0,
        _ => choose(n-1, k) + choose(n-1, k-1)
    }
}

pub const fn choose_two(n: usize) -> usize {
    n*(n+1)/2-n
}

pub fn pos_to_edge(pos: usize) -> Edge {
    let mut u = 0;
    let mut pos_u_n_minus_1 = N - 2;
    while pos_u_n_minus_1 < pos {
        pos_u_n_minus_1 += N - 2 - u;
        u += 1;
    }
    let v = N - 1 - (pos_u_n_minus_1 - pos);
    (u, v)
}

pub fn edge_to_pos((u, v): Edge) -> usize {
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
            assert_eq!(choose_two(n),
            choose(n, 2));
        }
    }

    #[test]
    fn pos_to_edge_test() {
        for (i, (u, v)) in (0..N).tuple_combinations().enumerate() {
            assert_eq!(pos_to_edge(i), (u, v));
        }
    }

    #[test]
    fn edge_to_pos_test() {
        for (i, (u, v)) in (0..N).tuple_combinations().enumerate() {
            assert_eq!(edge_to_pos((u,v)), i);
            assert_eq!(edge_to_pos((v,u)), i);
        }
    }
}

pub const C: Color = S.len();
pub const E: usize = choose_two(N);
pub struct ColoredGraph {
    neighborhoods: [[Uxx; N]; C]
}

pub fn random_edge(rng: &mut ThreadRng) -> Edge {
    let u = rng.gen_range(0..N);
    let v = rng.gen_range(0..N-1);
    if v < u { (v, u) } else { (u, v+1) }
}

impl ColoredGraph {
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
    
    pub fn red() -> ColoredGraph {
        let mut neighborhoods: [[Uxx; N]; C] = [[0; N]; C];
        for u in 0..N {
            let mut neighborhood = mask!([0..N], Uxx);
            unset!(in neighborhood, Uxx, u);
            neighborhoods[0][u] = neighborhood
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

    pub fn recolor(&mut self, recolor: Recoloring) {
        self.delete(recolor.old_color, recolor.edge);
        self.add(recolor.new_color, recolor.edge);
    }

    pub fn color(&self, (u, v): Edge) -> Option<Color> {
        match self.neighborhoods
            .iter()
            .position(|&neighborhood| 
            is_set!((neighborhood[u]), Uxx, v)) {
                Some(c) => Some(c),
                None => None
        }
    }

    pub fn bit_neighborhood(&self, color: Color, u: Vertex) -> Uxx {
        self.neighborhoods[color][u]
    }

    pub fn common_neighborhood(&self, color: Color, u: Vertex, v: Vertex) -> Uxx {
        self.bit_neighborhood(color, u) & 
        self.bit_neighborhood(color, v)
    }

    pub fn random_edge(&mut self, rng: &mut ThreadRng) -> ColoredEdge {
        let edge = random_edge(rng);
        let color = self.color(edge)
            .unwrap();
        ColoredEdge { color, edge }
    }

    pub fn show_matrix(&self) {
        if C == 0 { println!("Colorless graph!") }
        for u in 1..N {
            for v in 0..u {
                let c = self.color((u, v)).unwrap_or(C);
                print!("{c}");
            }
            print!("; ");
        }
    }

    pub fn show_neighborhoods(&self) {
        for u in 0..N {
            print!("vertex {u}:");
            for c in 0..C {
                print!(" {:?}", BitIter::from(self.bit_neighborhood(c, u)).collect::<Vec<_>>());
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_red_cliques() {
        let red = ColoredGraph::red();
        assert_eq!(choose(N, S[0]) as Iyy,
            red.count_cliques(0, None, None));
        for c in 1..C {
            assert_eq!(0,
                red.count_cliques(c, None, None))
        }
    }
}