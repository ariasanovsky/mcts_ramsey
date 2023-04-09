use crate::prelude::*;

pub trait Neighborhood:
    std::cmp::PartialEq + std::cmp::Eq + std::hash::Hash + 
    std::clone::Clone + std::default::Default
{
    fn add(&mut self, u: Vertex);
    fn remove(&mut self, u: Vertex);
    fn toggle(&mut self, u: Vertex);
}

type IntSet = Uxx;

impl Neighborhood for IntSet {
    fn add(&mut self, u: Vertex) {
        todo!()
    }

    fn remove(&mut self, u: Vertex) {
        todo!()
    }

    fn toggle(&mut self, u: Vertex) {
        todo!()
    }
}