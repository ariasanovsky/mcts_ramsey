use std::ops::BitAnd;

use bit_fiddler::{mask, unset, set, is_set, toggle};
use bit_iter::BitIter;

use crate::prelude::*;

pub trait Neighborhood:
    std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + 
    std::clone::Clone + std::marker::Copy + std::default::Default +
    std::ops::BitAnd<Output = Self>
{
    fn full() -> Self;
    fn interval_to_end(u: Vertex) -> Self;
    fn contains(&self, u: Vertex) -> bool;
    fn iter(&self) -> BitIter<usize>;
    fn n_elements(&self) -> u32;
    fn add(&mut self, u: Vertex);
    fn delete(&mut self, u: Vertex);
    fn toggle(&mut self, u: Vertex);
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct UxxN<const N: usize> {
    bits: Uxx
}

impl<const N: usize> BitAnd for UxxN<N> {
    type Output = UxxN<N>;

    fn bitand(self, rhs: Self) -> Self::Output {
        UxxN { bits: self.bits & rhs.bits }
    }
}

impl<const N: usize> Neighborhood for UxxN<N> {
    fn full() -> Self {
        UxxN { bits: mask!([0..N], Uxx) }
    }

    fn interval_to_end(u: Vertex) -> Self {
        UxxN { bits: mask!([u..N], Uxx) }
    }
    
    
    fn contains(&self, u: Vertex) -> bool {
        let me = self.bits;
        is_set!(me, Uxx, u)
    }

    fn iter(&self) -> BitIter<usize> {
        BitIter::<usize>::from(self.bits as usize)
    }

    fn n_elements(&self) -> u32 {
        self.bits.count_ones()
    }
    
    fn add(&mut self, u: Vertex) {
        let mut me = self.bits;
        set!(in me, Uxx, u);
        self.bits = me
    }

    fn delete(&mut self, u: Vertex) {
        let mut me = self.bits;
        unset!(in me, Uxx, u);
        self.bits = me
    }

    fn toggle(&mut self, u: Vertex) {
        let mut me = self.bits;
        toggle!(in me, Uxx, u);
        self.bits = me
    }
}
