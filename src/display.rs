use crate::{prelude::*, neighborhood::*};

use bit_fiddler::{is_set};
use itertools::Itertools;
use crate::colored_graph::*;


impl<T: Neighborhood, const C: usize, const N: usize>
ColoredGraph<T, C, N> {
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
                print!(" {:?}", self.bit_neighborhood(c, u).iter().collect::<Vec<_>>());
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

        let (mut word, mut pos): (u8, usize) = (0, 5);
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
    type T = super::Uxx;
    #[test]
    fn mckay_example() {
        let mut graph = ColoredGraph::<T, 2, 5>::red();
        graph.recolor(Recoloring{ old_color: 0, new_color: 1, edge: (0, 2) });
        graph.recolor(Recoloring{ old_color: 0, new_color: 1, edge: (0, 4) });
        graph.recolor(Recoloring{ old_color: 0, new_color: 1, edge: (1, 3) });
        graph.recolor(Recoloring{ old_color: 0, new_color: 1, edge: (3, 4) });
        assert_eq!(graph.graph6(1), String::from("DQc"))
    }
}

impl<T: Neighborhood, const C: usize, const N: usize>
TryFrom<&Vec<String>> for ColoredGraph<T, C, N> {
    type Error = String;

    fn try_from(strings: &Vec<String>) -> Result<Self, Self::Error> {
        let mut neighborhoods: [[T; N]; C] = [[T::default(); N]; C];
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
                    neighborhoods[c][u].add(v);
                    neighborhoods[c][v].add(u);
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
    
    type T = super::Uxx;

    #[test]
    fn red_graph() {
        let red = ColoredGraph::<T, C, N>::red();
        let strings = red.graph6s();
        let red2 = ColoredGraph::try_from(&strings);
        assert_eq!(Ok(red), red2)
    }

    #[test]
    fn random_recoloring() {
        let mut graph = ColoredGraph::<T, C, N>::red();
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

// temporarily lifting from plotters: https://docs.rs/plotters/latest/src/plotters/style/palette.rs.html#17
const COLORS: &[(u8, u8, u8)] = &[
    (230, 25, 75),
    (60, 180, 75),
    (255, 225, 25),
    (0, 130, 200),
    (245, 130, 48),
    (145, 30, 180),
    (70, 240, 240),
    (240, 50, 230),
    (210, 245, 60),
    (250, 190, 190),
    (0, 128, 128),
    (230, 190, 255),
    (170, 110, 40),
    (255, 250, 200),
    (128, 0, 0),
    (170, 255, 195),
    (128, 128, 0),
    (255, 215, 180),
    (0, 0, 128),
    (128, 128, 128),
    (0, 0, 0),
];


impl<T: Neighborhood, const C: usize, const N: usize>
ColoredGraph<T, C, N> {
    fn _tikz(&self) -> Vec<String> { // todo!("render at runtime w/ tectonic? best on linux...")
        let size_in_cm = ":2cm";
        let mut tikz = format!(
            "{}\n\
            {}\n\
            {}\n\
            {}\n\
            {}\n]\n\
            {}{}}} {{\n\
            {}{}{}{}\n\
            }};\n",
            r"\usetikzlibrary{positioning}",
            r"\begin{tikzpicture}[",
            r"  main node/.style={circle,draw},",
            r"  no edge/.style={dashed,draw},",
            r"  yes edge/.style={draw},",
            r"\foreach \Rotulo [count=\ci] in {0,...,",
            N-1,
            r"  \node[main node] (\ci) at (\ci*360/",
            N,
            size_in_cm,
            r") {\Rotulo};"
        );

        for (u, v) in (0..N).tuple_combinations() {
            let colored_edge = ColoredEdge { color: 0, edge: (u,v) };
            if self.has_edge(colored_edge) {
                tikz = format!("{tikz}\\draw ({}) [yes edge]-- ({});\n", u+1, v+1)
            }
            else {
                tikz = format!("{tikz}\\draw ({}) [no edge]-- ({});\n", u+1, v+1)
            }
        }

        tikz = format!(
            "{tikz}{}",
            r"\end{tikzpicture}"
        );

        vec!(tikz)
    }
}

#[test]
fn can_generate_tikz() {
    let mut rng = rand::thread_rng();
    type T = Uxx;
    let graph = ColoredGraph::<T, 2, 8>::uniformly_random(&mut rng);
    println!("{}", graph._tikz()[0]);
}

const fn usize_sqrt(n: usize) -> usize {
    let mut s = 0;
    while s*s < n {
        s += 1;
    }
    s
}

use svg::{
    Document,
    node::element::{Path, Circle, path::Data}
};

pub struct GraphPics {
    pics: Vec<Document>,
    name: String
}

impl GraphPics {
    pub fn render(&self) {
        for (c, pic) in self.pics.iter().enumerate() {
            svg::save(
                format!("plots/{}_{c}.svg", self.name), 
                pic)
            .unwrap();
        }
    }
}

impl<T: Neighborhood, const C: usize, const N: usize>
ColoredGraph<T, C, N> {
    pub fn svg(&self, name: String) -> GraphPics {
        let k: f64 = std::f64::consts::TAU / N as f64;
        let r: f64 = usize_sqrt(N) as f64;
        
        let pos: [(f64, f64); N] = std::array::from_fn(
            |i|
            {
                let theta = k * i as f64;
                let (sin, cos) = theta.sin_cos();
                (r*cos, r*sin)
            }
        );

        let pics = (0..C).map(|c| {
            let mut document = Document::new()
                .set("viewBox", (-1.1*r, -1.1*r, 2.2*r, 2.2*r));
            
            for (u, v) in (0..N).tuple_combinations() {
                let colored_edge = ColoredEdge { color: c, edge: (u, v) };
                if self.has_edge(colored_edge) {
                    let data = Data::new()
                        .move_to(pos[u])
                        .line_to(pos[v]);

                    let (red, green, blue) = COLORS[c];

                    let path = Path::new()
                        .set("fill", "none")
                        .set("stroke", format!("rgb({red}, {green}, {blue})"))
                        .set("stroke-width", 0.05)
                        .set("d", data);

                    document = document.clone().add(path); 
                }   
            }

            for (x, y) in pos {
                let node = Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", 0.1);
                document = document.clone().add(node);
            }
            document
        }).collect();
        GraphPics { pics, name }
    }
}

#[test]
fn can_generate_svg() {
    const C: usize = 3;
    const N: usize = 8;
    type T = Uxx;
    
    let mut rng = rand::thread_rng();
    let graph = ColoredGraph::<T, C, N>::uniformly_random(&mut rng);
    let docs = graph.svg(String::from("test"));
    docs.render()
}