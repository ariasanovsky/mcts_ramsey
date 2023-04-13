#[cfg(test)]
mod action_matrix_initialization {
    use crate::{prelude::{choose, choose_two, S}, colored_graph::{neighborhood::UxxN, ColoredGraph}, action_matrix::ActionMatrix};
    
    const C: usize = 2;
    const N: usize = 8;
    const E: usize = choose_two(N);

    type T = UxxN<N>;

    #[test]
    fn correct_number_of_acounts() {
        let graph = ColoredGraph::<T, C, N>::red();
        let actions = ActionMatrix::<T, C, N, E>::from(graph);
        assert_eq!(actions.actions.len(), (C-1) * E)
    }

    #[test]
    fn red_graph_action_gradients() {
        let graph = ColoredGraph::<T, C, N>::red();
        let actions = ActionMatrix::<T, C, N, E>::from(graph);
        for ((color, _), slope) in actions.actions {
            assert_ne!(color, 0);
            assert_eq!(slope, choose(N-2, S[0]-2));
        }
    }
}

#[cfg(test)]
mod recolor_gradient_test {
    use itertools::Itertools;

    use crate::{prelude::{choose, choose_two, S}, colored_graph::{neighborhood::UxxN, ColoredGraph}, action_matrix::ActionMatrix};
    
    const C: usize = 2;
    const N: usize = 8;
    const E: usize = choose_two(N);

    type T = UxxN<N>;

    #[test]
    fn one_recoloring() {
        let mut actions = ActionMatrix::<T, C, N, E>::from(ColoredGraph::<T, C, N>::red());
        actions.recolor((1, 0), 0);
        for (i, (u,v)) in (0..N).tuple_combinations().enumerate() {
            let slope_0 = actions.slope((0, i));
            let slope_1 = actions.slope((1, i));
            
            match (u, v) {
                (0, 1) => assert_eq!(slope_1, None),
                (0, _) | (1, _) => assert_eq!(slope_1, Some(&choose(N-3, S[0]-2))),
                (_, _) => assert_eq!(slope_1, Some(&(
                    choose(N-2, S[0]-2) - 
                        if N >= 4 && S[0] >= 4 { choose(N-4, S[0].checked_sub(4).unwrap()) }
                        else { 0 }
                ))),
            }

            match (u, v) {
                (0, 1) => assert_eq!(slope_0, Some(&-choose(N-2, S[0]-2))),
                _ => assert_eq!(slope_0, None)
            }
        }
    }
}

#[cfg(test)]
mod test_random_recoloring {
    use itertools::Itertools;

    use crate::{prelude::{choose, choose_two, S, Iyy, pos_to_edge}, colored_graph::{neighborhood::{UxxN, Neighborhood}, ColoredGraph, ColoredEdge}, action_matrix::{ActionMatrix, Action}};
    
    const C: usize = 2;
    const N: usize = 8;
    const E: usize = choose_two(N);

    type T = UxxN<N>;

    #[test]
    fn consistent_counts() {
        let mut actions = ActionMatrix::<T, C, N, E>::from(ColoredGraph::<T, C, N>::red());
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            for c in 0..C {
                let graph_count = actions.graph.count_cliques(c, None, None);
                let matrix_count: Iyy = (0..N)
                    .tuple_combinations()
                    .enumerate()
                    .filter(
                        |(_, (u, v))|
                        {
                            let colored_edge = ColoredEdge { color: c, edge: (*u,*v) };
                            actions.graph.has_edge(colored_edge)
                        })
                    .map(|(pos, _)| {
                        actions.counts[c][pos]
                    })
                    .sum();
                assert_eq!(graph_count * choose(S[c], 2), matrix_count);
            }
            actions.randomly_act(&mut rng);
        }
    }

    impl<T: Neighborhood> ActionMatrix<T, C, N, E> {
        fn _calculate_slope(&self, (new_color, pos): Action) -> Option<Iyy> {
            let edge = pos_to_edge::<N>(pos);
            let old_color = self.graph.color(edge)
                .unwrap();
        
            if old_color == new_color { None } 
            else { Some(
                self.counts[old_color][pos] -
                self.counts[new_color][pos]
            )}
        }
    }

    #[test]
    fn verify_all_slopes() {
        let mut actions: ActionMatrix<T, C, N, E> = ActionMatrix::from(ColoredGraph::red());
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            for c in 0..C {
                for pos in 0..E {
                    let slope = actions.slope((c, pos))
                        .map(|&x| x);
                    let calculated_slope = actions._calculate_slope((c, pos));
                    assert_eq!(slope, calculated_slope)
                }
            }
            actions.randomly_act(&mut rng)
        }
    }

    #[test]
    fn consistent_scores() {
        let mut actions = ActionMatrix::<T, C, N, E>::from(ColoredGraph::<T, C, N>::red());
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            assert_eq!(actions.score(), actions.graph.score());
            assert_eq!(actions.score(), actions.total());
            actions.randomly_act(&mut rng)
        }
    }
}
