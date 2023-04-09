use bit_fiddler::{mask, unset, set, is_set, toggle};
use bit_iter::BitIter;

use crate::prelude::*;

pub trait Neighborhood:
    std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + 
    std::clone::Clone + std::marker::Copy + std::default::Default +
    std::ops::BitAnd<Output = Self>
{
    fn full() -> Self;
    fn contains(&self, u: Vertex) -> bool;
    fn iter(&self) -> BitIter<usize>;
    fn n_elements(&self) -> u32;
    fn add(&mut self, u: Vertex);
    fn delete(&mut self, u: Vertex);
    fn toggle(&mut self, u: Vertex);
}

impl Neighborhood for Uxx {
    fn full() -> Self {
        mask!([0..N], Uxx)
    }

    fn contains(&self, u: Vertex) -> bool {
        let me = *self;
        is_set!(me, Uxx, u)
    }

    fn iter(&self) -> BitIter<usize> {
        BitIter::<usize>::from(*self as usize)
    }

    fn n_elements(&self) -> u32 {
        self.count_ones()
    }
    
    fn add(&mut self, u: Vertex) {
        let mut me = *self;
        set!(in me, Uxx, u);
        *self = me
    }

    fn delete(&mut self, u: Vertex) {
        let mut me = *self;
        unset!(in me, Uxx, u);
        *self = me
    }

    fn toggle(&mut self, u: Vertex) {
        let mut me = *self;
        toggle!(in me, Uxx, u);
        *self = me
    }
}