use bit_iter::BitIter;
use crate::colored_graph::*;

impl<const N: usize> ColoredGraph<N> {
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

        let (mut word, mut ctr) = (0 as u8, 0 as usize);
        for v in 1..N { // colex
            for u in 0..v {
                let colored_edge = ColoredEdge {color, edge: (v, u)};
                if self.has_edge(colored_edge) {
                    word |= 1;
                }
                if ctr == 5 {
                    graph.push((word + 63) as char);
                    ctr = 0
                }
                else {
                    word >>= 1;
                    ctr += 1
                }
            }
        }

        if ctr != 0 {
            word >>= 6 - ctr;
            graph.push((word + 63) as char)
        }

        graph
    }
}

#[cfg(test)]
mod g6_tests {

}