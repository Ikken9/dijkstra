use uuid::Uuid;
use crate::edge::Edge;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct VertexId(pub(crate) Uuid);

pub struct Vertex {
    pub id: VertexId,
    pub edges: Vec<Edge>
}

