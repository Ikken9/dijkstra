use crate::edge::Edge;
use crate::graph::Graph;
use crate::vertex::{Vertex, VertexId};

mod graph;
mod vertex;
mod edge;

fn main() {
    let edge_a_b = Edge { to: VertexId('b'), weight: 2 };
    let edge_b_a = Edge { to: VertexId('a'), weight: 2 };
    let edge_a_c = Edge { to: VertexId('c'), weight: 6 };
    let edge_c_a = Edge { to: VertexId('a'), weight: 6 };
    let edge_b_d = Edge { to: VertexId('d'), weight: 5 };
    let edge_d_b = Edge { to: VertexId('b'), weight: 5 };
    let edge_c_d = Edge { to: VertexId('d'), weight: 8 };
    let edge_d_c = Edge { to: VertexId('c'), weight: 8 };
    let edge_d_e = Edge { to: VertexId('e'), weight: 1 };
    let edge_d_g = Edge { to: VertexId('g'), weight: 4 };
    let edge_e_g = Edge { to: VertexId('g'), weight: 2 };
    let edge_e_f = Edge { to: VertexId('f'), weight: 6 };
    let edge_g_f = Edge { to: VertexId('f'), weight: 3 };

    let vertex_a = Vertex { id: VertexId('a'), edges: vec![edge_a_b, edge_b_a, edge_a_c, edge_c_a] };
    let vertex_b = Vertex { id: VertexId('b'), edges: vec![edge_b_a, edge_a_b, edge_b_d, edge_d_b] };
    let vertex_c = Vertex { id: VertexId('c'), edges: vec![edge_c_a, edge_a_c, edge_c_d, edge_d_c] };
    let vertex_d = Vertex { id: VertexId('d'), edges: vec![edge_d_b, edge_b_d, edge_d_c, edge_c_d, edge_d_e, edge_d_g] };
    let vertex_e = Vertex { id: VertexId('e'), edges: vec![edge_e_f, edge_e_g] };
    let vertex_f = Vertex { id: VertexId('f'), edges: vec![] };
    let vertex_g = Vertex { id: VertexId('g'), edges: vec![edge_g_f] };

    let mut graph = Graph::new();
    graph.add_vertex(vertex_a.clone());
    graph.add_vertex(vertex_b);
    graph.add_vertex(vertex_c);
    graph.add_vertex(vertex_d);
    graph.add_vertex(vertex_e);
    graph.add_vertex(vertex_f);
    graph.add_vertex(vertex_g);

    println!("\nDijkstra's Algorithm implemented using a priority queue");
    graph.dijkstra_heap(vertex_a.clone());

    println!("\nDijkstra's Algorithm implemented without using a priority queue");
    graph.dijkstra_no_heap(vertex_a.clone());
}