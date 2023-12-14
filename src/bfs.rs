// This module is the implementation of the Breadth-First Search algorithm

pub mod bfs {
    use std::collections::{HashSet, VecDeque};

    // Perform BFS on a graph and returns the distance between start and end nodes
    // graph: adjacency list representation of the graph
    // start: index of the start node; end: index of the end node

    pub fn bfs(graph: &Vec<Vec<usize>>, start: usize, end: usize) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        // Vector to keep track of visited nodes
        let mut visited = vec![false; graph.len()];
        // Queue to manage nodes during BFS traversal
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((current, depth)) = queue.pop_front() {
            if current == end {
                return Some(depth);
            }
            if !visited[current] {
                visited[current] = true;
                for &neighbor in &graph[current] {
                    if !visited[neighbor] {
                        queue.push_back((neighbor, depth + 1));
                    }
                }
            }
        }

        None
    }

}