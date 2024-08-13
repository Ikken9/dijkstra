use std::collections::{BinaryHeap, HashMap, HashSet};
use uuid::Uuid;
use crate::edge::Edge;
use crate::vertex::{Vertex, VertexId};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: VertexId,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Graph {
    pub vertices: HashMap<VertexId, Vertex>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new()
        }
    }

    pub fn add_vertex(&mut self) {
        let id = VertexId(Uuid::new_v4());
        let vertex = Vertex { id, edges: Vec::new() };
        self.vertices.insert(id, vertex);
    }

    pub fn add_edge(&mut self, starting_vertex: VertexId, ending_vertex: VertexId, weight: u32) {
        if let Some(v) = self.vertices.get_mut(&starting_vertex) {
            v.edges.push( Edge {
                dest: ending_vertex,
                weight
            })
        }
    }

    pub fn dijkstra(&self, start: VertexId, goal: VertexId) -> Option<Vec<VertexId>> {
        let mut dist: HashMap<VertexId, u32> = HashMap::new();
        let mut prev: HashMap<VertexId, Option<VertexId>> = HashMap::new();
        let mut unvisited: HashSet<VertexId> = HashSet::new();

        for node_id in self.vertices.keys() {
            dist.insert(*node_id, u32::MAX);
            prev.insert(*node_id, None);
            unvisited.insert(*node_id);
        }
        dist.insert(start, 0);

        while !unvisited.is_empty() {
            let current = match unvisited.iter().min_by_key(|&&node_id| dist.get(&node_id).unwrap_or(&u32::MAX)) {
                Some(&node_id) => node_id,
                None => break,
            };

            if current == goal {
                let mut path = Vec::new();
                let mut u = goal;
                while let Some(prev_node) = prev.get(&u).and_then(|&p| p) {
                    path.push(u);
                    u = prev_node;
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            unvisited.remove(&current);

            if let Some(node) = self.vertices.get(&current) {
                for edge in &node.edges {
                    let next = edge.dest;
                    let new_dist = dist.get(&current).unwrap_or(&u32::MAX) + edge.weight;

                    if new_dist < *dist.get(&next).unwrap_or(&u32::MAX) {
                        dist.insert(next, new_dist);
                        prev.insert(next, Some(current));
                    }
                }
            }
        }

        None
    }

    fn dijkstra_heap(&self, start: VertexId, goal: VertexId) -> Option<Vec<VertexId>> {
        let mut dist: HashMap<VertexId, u32> = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut predecessors: HashMap<VertexId, VertexId> = HashMap::new();

        dist.insert(start, 0);
        heap.push(State { cost: 0, position: start });

        while let Some(State { cost, position }) = heap.pop() {
            if position == goal {
                let mut path = Vec::new();
                let mut current = goal;
                while let Some(&prev) = predecessors.get(&current) {
                    path.push(current);
                    current = prev;
                }
                path.push(start);
                path.reverse();
                return Some(path);
            }

            if cost > *dist.get(&position).unwrap_or(&u32::MAX) {
                continue;
            }

            if let Some(node) = self.vertices.get(&position) {
                for edge in &node.edges {
                    let next = State {
                        cost: cost + edge.weight,
                        position: edge.dest,
                    };

                    if next.cost < *dist.get(&next.position).unwrap_or(&u32::MAX) {
                        heap.push(next);
                        dist.insert(next.position, next.cost);
                        predecessors.insert(next.position, position);
                    }
                }
            }
        }

        None
    }
}