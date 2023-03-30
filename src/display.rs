use bit_fiddler::{is_set, set};
use bit_iter::BitIter;
use crate::colored_graph::*;


impl<const C: usize, const N: usize> ColoredGraph<C, N> {
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

    pub fn graph6(&self, color: Color) -> String {
        let mut graph = match N.cmp(&62) {
            std::cmp::Ordering::Less | 
            std::cmp::Ordering::Equal => String::from((N+63) as u8 as char),
            std::cmp::Ordering::Greater => todo!("large N g6 prefix")
        };

        let (mut word, mut pos) = (0 as u8, 5 as usize);
        for (u, v) in (0..N).flat_map(|v| (0..v).map(move |u| (u, v))) {
            let colored_edge = ColoredEdge {color, edge: (v, u)};
            if self.has_edge(colored_edge) {
                word = bit_fiddler::set!(word, u8, pos);
            }
            if pos == 0 {
                graph.push((word + 63) as char);
                word = 0;
                pos = 5
            }
            else {
                pos -= 1
            }
        }
        
        if pos != 5 {
            graph.push((word + 63) as char)
        }

        graph
    }

    pub fn graph6s(&self) -> Vec<String> {
        (0..C).map(|c| self.graph6(c)).collect()
    }
}

#[cfg(test)]
mod g6_tests {
    use crate::colored_graph::{ColoredGraph, Recoloring};

    #[test]
    fn mckay_example() {
        let mut graph = ColoredGraph::<2, 5>::red();
        graph.recolor(Recoloring{ old_color: 0, new_color: 1, edge: (0, 2) });
        graph.recolor(Recoloring{ old_color: 0, new_color: 1, edge: (0, 4) });
        graph.recolor(Recoloring{ old_color: 0, new_color: 1, edge: (1, 3) });
        graph.recolor(Recoloring{ old_color: 0, new_color: 1, edge: (3, 4) });
        assert_eq!(graph.graph6(1), String::from("DQc"))
    }
}

impl<const C: usize, const N: usize> TryFrom<&Vec<String>> for ColoredGraph<C, N> {
    type Error = String;

    fn try_from(strings: &Vec<String>) -> Result<Self, Self::Error> {
        let mut neighborhoods: [[Uxx; N]; C] = [[0; N]; C];
        for (c, string) in strings.iter().enumerate() {
            let n = string.as_bytes()[0] - 63;
            if n != (N as u8) { return Err(format!("{n} != {N}")) }
            if n >= 63 { todo!("{N} >= 63") }
            let mut pos: u8 = 5;
            let mut i = 1;
            let mut curr_char = string.as_bytes()[i] - 63;
            for (u, v) in (0..N).flat_map(|v| (0..v).map(move |u| (u, v))) {
                if pos == 5 {
                    curr_char = string.as_bytes()[i] - 63;
                }
                if is_set!(curr_char, u8, pos) {
                    neighborhoods[c][u] = set!((neighborhoods[c][u]), Uxx, v);
                    neighborhoods[c][v] = set!((neighborhoods[c][v]), Uxx, u);
                }
                if pos == 0 {
                    pos = 5;
                    i += 1
                }
                else {
                    pos -= 1
                }
            }
        }
        Ok ( ColoredGraph::from(neighborhoods) )
    }
}

#[cfg(test)]
mod g6_graph_conversion_tests {
    use crate::colored_graph::ColoredGraph;
    
    const C: usize = 2;
    const N: usize = 8;
    
    #[test]
    fn red_graph() {
        let red = ColoredGraph::<C, N>::red();
        let strings = red.graph6s();
        let red2 = ColoredGraph::try_from(&strings);
        assert_eq!(Ok(red), red2)
    }

    #[test]
    fn random_recoloring() {
        let mut graph = ColoredGraph::<C, N>::red();
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            graph.show_neighborhoods();
            let strings = graph.graph6s();
            dbg!(&strings);
            let red2 = ColoredGraph::try_from(&strings);
            assert!(red2.is_ok());
            assert_eq!(&graph, &red2.unwrap());
            graph.randomly_recolor(&mut rng)
        }
    }
}