use crate::neighborhood::*;
use crate::prelude::*;

use itertools::Itertools;
use rand::prelude::*;
use rand::distributions::WeightedIndex;

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

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct ColoredGraph<T: Neighborhood, const C: usize, const N: usize> {
    neighborhoods: [[T; N]; C]
}

pub fn random_edge<const N: usize>(rng: &mut ThreadRng) -> Edge {
    let u = rng.gen_range(0..N);
    let v = rng.gen_range(0..N-1);
    if v < u { (v, u) } else { (u, v+1) }
}

impl<T: Neighborhood, const C: usize, const N: usize>
ColoredGraph<T, C, N> {
    pub fn score(&self) -> Iyy {
        (0..C)
        .map(|c| self.count_cliques(c, None, None))
        .sum()
    }
    
    pub fn count_cliques(&self, color: Color, s: Option<usize>, candidates: Option<T>) -> Iyy {
        let s = s.unwrap_or(S[color]);
        if s == 0 { return 1 }
        
        let candidates = candidates.unwrap_or(T::default());
        if s == 1 { return candidates.n_elements() as Iyy } 
        
        candidates.iter()
            .map(|u| candidates & self.bit_neighborhood(color, u) & T::full())
            .map(|candidates| self.count_cliques(color, Some(s-1), Some(candidates)))
            .sum()
    }

    pub fn count_edge_cliques(&self, color: Color, (u, v): Edge) -> Iyy {
        let candidates = Some(
            self.common_neighborhood(color, u, v));
        self.count_cliques(color, Some(S[color]-2), candidates)
    }
    
    pub fn red() -> ColoredGraph<T, C, N> {
        let mut neighborhoods: [[T; N]; C] = [[T::full(); N]; C];
        for u in 0..N {
            neighborhoods[0][u].delete(u)
        }
        ColoredGraph { neighborhoods }
    }

    pub fn uniformly_random(rng: &mut ThreadRng) -> ColoredGraph<T, C, N> {
        let mut neighborhoods: [[T; N]; C] = [[T::default(); N]; C];
        for (u, v) in (0..N).tuple_combinations() {
            let c = rng.gen_range(0..C);
            neighborhoods[c][u].add(v);
            neighborhoods[c][v].add(u)
        }
        
        ColoredGraph { neighborhoods }
    }

    pub fn random(rng: &mut ThreadRng, dist: &WeightedIndex<f64>) -> ColoredGraph<T, C, N> {
        let mut neighborhoods: [[T; N]; C] = [[T::default(); N]; C];
        for (u, v) in (0..N).tuple_combinations() {
            let c = rng.sample(dist);
            neighborhoods[c][u].add(v);
            neighborhoods[c][v].add(u)
        }
        
        ColoredGraph { neighborhoods }
    }

    fn add(&mut self, c: Color, (u,v): Edge) {
        self.neighborhoods[c][u].add(v);
        self.neighborhoods[c][v].add(u)
    }

    fn delete(&mut self, c: Color, (u,v): Edge) {
        self.neighborhoods[c][u].delete(v);
        self.neighborhoods[c][v].delete(u)
    }

    pub fn recolor(&mut self, recolor: Recoloring<N>) {
        self.delete(recolor.old_color, recolor.edge);
        self.add(recolor.new_color, recolor.edge);
    }

    pub fn has_edge(&self, colored_edge: ColoredEdge) -> bool {
        let ColoredEdge { color, edge: (u, v) } = colored_edge;
        self.bit_neighborhood(color, u).contains(v)
    }

    pub fn color(&self, (u, v): Edge) -> Option<Color> {
        self.neighborhoods
            .iter()
            .position(
                |neighborhood| 
                neighborhood[u].contains(v))
    }

    pub fn bit_neighborhood(&self, color: Color, u: Vertex) -> T {
        self.neighborhoods[color][u]
    }

    pub fn common_neighborhood(&self, color: Color, u: Vertex, v: Vertex) -> T {
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
    use crate::prelude::choose;

    use super::*;

    type T = Uxx;

    #[test]
    fn only_red_cliques() {
        const N: usize = 8;
        const C: usize = 2;
        let red = ColoredGraph::<T, C, N>::red();
        assert_eq!(choose(N, S[0]),
            red.count_cliques(0, None, None));
        for c in 1..C {
            assert_eq!(0,
                red.count_cliques(c, None, None))
        }
    }
}

impl<T: Neighborhood, const C: usize, const N: usize>
From<[[T; N]; C]> for ColoredGraph<T, C, N> {
    fn from(neighborhoods: [[T; N]; C]) -> Self {
        ColoredGraph { neighborhoods }
    }
}