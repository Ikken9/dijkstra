use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::vertex::{Vertex, VertexId};


pub struct Graph {
    pub vertices: HashMap<VertexId, Vertex>
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new()
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        let copy = vertex.clone();
        let id = copy.id;
        self.vertices.insert(id, copy);
    }

    pub fn dijkstra_heap(&mut self, start: Vertex) {
        let mut distances: HashMap<VertexId, u32> = HashMap::new();
        let mut visited: HashSet<VertexId> = HashSet::new();

        let mut priority_queue = BinaryHeap::new();

        distances.insert(start.id.clone(), 0);
        priority_queue.push(State { vertex: start.id, cost: 0 });

        while let Some(State { vertex: current_vertex, cost: current_distance }) = priority_queue.pop() {
            if !visited.insert(current_vertex) {
                continue;
            }

            println!("heap {:?}", visited);

            for neighbor in &self.vertices.get(&current_vertex).unwrap().edges {
                if let Some(next) = self.vertices.get(&neighbor.to) {
                    let distance = current_distance + neighbor.weight;
                    if distance < *distances.get(&neighbor.to).unwrap_or(&u32::MAX) {
                        distances.insert(neighbor.to.clone(), distance);

                        priority_queue.push(State { vertex: next.id, cost: distance });
                    }
                }
            }
        }

        for (to, cost) in distances {
            println!("\nFrom vertex \'{}\' to vertex \'{}\':", start.id, to);
            println!("  Cost: {}", cost);
        }
    }

    pub fn dijkstra_no_heap(&mut self, start: Vertex) {
        let mut distances: HashMap<VertexId, u32> = HashMap::new();
        let mut visited: HashSet<VertexId> = HashSet::new();

        distances.insert(start.id, 0);

        let mut current_vertex = start.clone();
        let graph_size = self.vertices.keys().len();

        while visited.len() < graph_size {
            visited.insert(current_vertex.id);
            println!("no heap {:?}", visited);
            let current_distance = *distances.get(&current_vertex.id).unwrap_or(&u32::MAX);

            for neighbor in &current_vertex.edges {
                let distance = current_distance + neighbor.weight;

                if distance < *distances.get(&neighbor.to).unwrap_or(&u32::MAX) {
                    distances.insert(neighbor.to, distance);
                }
            }

            let next_vertex = self.vertices
                .iter()
                .filter(|(_, v)| !visited.contains(&v.id))
                .min_by_key(|(_, v)| distances.get(&v.id).unwrap_or(&u32::MAX))
                .map(|(_, v)| v.clone());

            match next_vertex {
                Some(v) => current_vertex = v,
                None => break,
            }
        }

        for (to, cost) in distances {
            println!("\nFrom vertex \'{}\' to vertex \'{}\':", start.id, to);
            println!("  Cost: {}", cost);
        }
    }
}

#[derive(Eq, PartialEq)]
struct State {
    vertex: VertexId,
    cost: u32
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}