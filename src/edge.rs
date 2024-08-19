use crate::vertex::VertexId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    pub to: VertexId,
    pub weight: u32
}