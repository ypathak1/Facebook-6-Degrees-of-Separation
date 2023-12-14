// This module handles the processing of CSV files.

pub mod csv_processor {
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    // Reads a CSV file and converts it into a graph representation
    // Returns a tuple of (list of nodes, adjacency list of the graph)
    pub fn process_csv(file_path: &str) -> Result<(Vec<String>, Vec<Vec<usize>>), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);

        let mut nodes = Vec::new(); // List of unique nodes
        let mut edges_map = HashMap::new(); // Maps each node to its adjacent nodes
        let mut line_number = 0; // Track the line number for skipping the header

        // Iterate over each line in the CSV file
        for line in reader.lines() {
            // Read the line and handle any potential errors
            let line = line?;

            // Skip the first line (CSV header) by checking the line number
            if line_number > 0 { // Skipping the CSV header.
                let columns: Vec<&str> = line.split(',').collect();
                // Check if the line has exactly 2 columns (for a pair of nodes)
                // If not, return an error indicating an invalid CSV format
                if columns.len() != 2 {
                    return Err(Box::new(io::Error::new(io::ErrorKind::InvalidData, "Invalid CSV format")));
                }

                // Extract the node and edge from the columns
                let (node, edge) = (columns[0].to_string(), columns[1].to_string());

                // Check if the node is already in the edges_map
                if !edges_map.contains_key(&node) {
                    // Add the node to the nodes list and initialize its adjacency list in edges_map
                    nodes.push(node.clone());
                    edges_map.insert(node.clone(), Vec::new());
                }

                // Repeat the check and addition for the edge
                if !edges_map.contains_key(&edge) {
                    nodes.push(edge.clone());
                    edges_map.insert(edge.clone(), Vec::new());
                }

                // Find the index of the node and edge in the nodes list
                // These indices are used to represent the graph internally
                let node_index = nodes.iter().position(|n| n == &node).unwrap();
                let edge_index = nodes.iter().position(|n| n == &edge).unwrap();
                // Add the edge index to the adjacency list of the node in the edges_map
                edges_map.get_mut(&node).unwrap().push(edge_index);
            }
            // Increment the line number for the next iteration
            line_number += 1;
        }


        // Convert the edge map to an adjacency list for the graph
        let graph = nodes.iter().map(|node| edges_map[node].clone()).collect();
        Ok((nodes, graph))
    }
}
