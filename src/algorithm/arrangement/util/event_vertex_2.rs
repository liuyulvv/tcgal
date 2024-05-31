use std::{cell::RefCell, rc::Rc};

use crate::kernel::{edge_2::Edge2, number_type::NumberType, vertex_2::Vertex2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventVertex2Type {
    Start,
    End,
    Intersection,
}

#[derive(Debug, Clone)]
pub struct EventVertex2<NT: NumberType> {
    vertex: Rc<RefCell<Vertex2<NT>>>,
    vertex_type: EventVertex2Type,
    edges: Vec<Rc<RefCell<Edge2<NT>>>>,
}

impl<NT: NumberType> EventVertex2<NT> {
    pub fn new(vertex: Rc<RefCell<Vertex2<NT>>>, vertex_type: EventVertex2Type) -> Self {
        Self {
            vertex,
            vertex_type,
            edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Rc<RefCell<Edge2<NT>>>) {
        self.edges.push(edge);
    }

    pub fn vertex(&self) -> Rc<RefCell<Vertex2<NT>>> {
        self.vertex.clone()
    }

    pub fn vertex_type(&self) -> EventVertex2Type {
        self.vertex_type.clone()
    }

    pub fn edges(&self) -> Vec<Rc<RefCell<Edge2<NT>>>> {
        self.edges.clone()
    }
}

impl<NT: NumberType> PartialEq for EventVertex2<NT> {
    fn eq(&self, other: &Self) -> bool {
        let self_vertex = self.vertex.borrow();
        let other_vertex = other.vertex.borrow();
        self_vertex.equals(&other_vertex)
    }
}

impl<Nt: NumberType> Eq for EventVertex2<Nt> {}

impl<NT: NumberType> Ord for EventVertex2<NT> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_vertex = self.vertex.borrow();
        let other_vertex = other.vertex.borrow();
        if self_vertex.equals(&other_vertex) {
            return std::cmp::Ordering::Equal;
        }
        if self_vertex.y() > other_vertex.y() {
            std::cmp::Ordering::Greater
        } else if self_vertex.y() < other_vertex.y() {
            std::cmp::Ordering::Less
        } else {
            if self_vertex.x() > other_vertex.x() {
                std::cmp::Ordering::Greater
            } else if self_vertex.x() < other_vertex.x() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}

impl<NT: NumberType> PartialOrd for EventVertex2<NT> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_vertex = self.vertex.borrow();
        let other_vertex = other.vertex.borrow();
        if self_vertex.equals(&other_vertex) {
            return Some(std::cmp::Ordering::Equal);
        }
        if self_vertex.y() > other_vertex.y() {
            Some(std::cmp::Ordering::Greater)
        } else if self_vertex.y() < other_vertex.y() {
            Some(std::cmp::Ordering::Less)
        } else {
            if self_vertex.x() > other_vertex.x() {
                Some(std::cmp::Ordering::Greater)
            } else if self_vertex.x() < other_vertex.x() {
                Some(std::cmp::Ordering::Less)
            } else {
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}
