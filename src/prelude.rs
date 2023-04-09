include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub type Color = usize;
pub type Vertex = usize;
pub type Edge = (Vertex, Vertex);

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
