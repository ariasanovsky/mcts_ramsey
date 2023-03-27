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
        for v in 1..N { // colex
            for u in 0..v {
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
        }

        if pos != 5 {
            graph.push((word + 63) as char)
        }

        graph
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