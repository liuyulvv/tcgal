use crate::{
    kernel::base_dcel::base_vertex_2::BaseVertex2,
    number_type::base_number_type_trait::BaseNumberTypeTrait,
};

use super::edge_2::Edge2;

#[derive(Debug, Clone)]
pub struct Vertex2<'a, NT: BaseNumberTypeTrait> {
    x: NT,
    y: NT,
    edges: Vec<&'a Edge2<'a, NT>>,
}

impl<'a, NT: BaseNumberTypeTrait> BaseVertex2<'a, NT> for Vertex2<'a, NT> {
    type Edge = Edge2<'a, NT>;

    fn new(x: NT, y: NT) -> Self {
        Self {
            x,
            y,
            edges: Vec::new(),
        }
    }

    fn x(&self) -> NT {
        self.x
    }

    fn y(&self) -> NT {
        self.y
    }

    fn edges(&self) -> Vec<&Self::Edge> {
        self.edges.clone()
    }

    fn add_edge(&mut self, edge: &'a Self::Edge) {
        self.edges.push(edge);
    }

    fn remove_edge(&mut self, edge: &'a Self::Edge) {
        self.edges.retain(|e| e != &edge);
    }
}

impl<'a, NT: BaseNumberTypeTrait> PartialEq for Vertex2<'a, NT> {
    fn eq(&self, other: &Self) -> bool {
        // self.x == other.x && self.y == other.y
        true
    }
}
