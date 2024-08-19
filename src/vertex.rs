use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::edge::Edge;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct VertexId(pub(crate) char);

impl Ord for VertexId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for VertexId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for VertexId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub id: VertexId,
    pub edges: Vec<Edge>
}

impl PartialOrd<Self> for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        other.id.cmp(&self.id)
    }
}

