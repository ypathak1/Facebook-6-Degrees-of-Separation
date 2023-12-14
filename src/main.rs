mod bfs;
mod processor;

use bfs::bfs::*;
use processor::csv_processor::*;
use std::collections::HashMap;

fn main() {
    let file_path = "facebook_combined.csv";
    // Process the CSV file to get nodes and the graph's adjacency list
    let (nodes, graph) = match process_csv(file_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to process CSV file: {}", e);
            return;
        }
    };
    

    // hashmap to count the number of connections at each distance
    let mut distance_counts = HashMap::new();
        
    // Variables to calculate mean, max distance, and standard deviation
    let mut total_distances = 0;
    let mut max_distance = 0;
    let mut sum_distances = 0.0;

    // Iterate over all pairs of nodes to compute BFS distances
    for (i, _) in nodes.iter().enumerate() {
        for (j, _) in nodes.iter().enumerate() {
            if i != j {
                if let Some(distance) = bfs(&graph, i, j) {
                    *distance_counts.entry(distance).or_insert(0) += 1;
                    total_distances += 1;
                    sum_distances += distance as f64;
                    max_distance = max_distance.max(distance);
                }
            }
        }
    }

     // Calculate mean separation distance
    let mean_distance = sum_distances / total_distances as f64;
    // Prepare to calculate standard deviation
    let mut sum_of_squares = 0.0;

    // Print the percentage for each degree of separation
    for distance in 1..=max_distance {
        let count = distance_counts.get(&distance).copied().unwrap_or(0);
        println!("The connections with {} degrees of separation are {:.2}% of the valid connections.",
                 distance,
                 (count as f64 / total_distances as f64) * 100.0);
        sum_of_squares += (count as f64) * ((distance as f64 - mean_distance).powi(2));
    }

    // calculate variance
    let variance = sum_of_squares / total_distances as f64;
    // Calculate and print the standard deviation of separation distances
    let standard_deviation = variance.sqrt();

    // Calculate the percentage of connections within six degrees of separation
    let percentage_six_degrees = calculate_percentage_within_six_degrees(&distance_counts, total_distances);

    println!("Mean separation: {:.2}", mean_distance);
    println!("Max distance: {}", max_distance);
    println!("Standard deviation of separation: {:.2}", standard_deviation);
    println!("The percentage of connected nodes that can be reached within six degrees of separation is {:.2}%", percentage_six_degrees);
}

// Function to calculate the percentage of connections within six degrees of separation
fn calculate_percentage_within_six_degrees(distance_counts: &HashMap<usize, usize>, total_distances: usize) -> f64 {
    let sum_six_degrees_or_less = distance_counts.iter()
        .filter(|(&k, _)| k <= 6)
        .map(|(_, &v)| v)
        .sum::<usize>() as f64;

    sum_six_degrees_or_less / total_distances as f64 * 100.0
}


// Tests

#[cfg(test)]
mod tests {
    use super::*;

    // Test to check self-connection (node to itself)
    #[test]
    fn check_self_connection() {
        let adjacency_test: Vec<Vec<usize>> = vec![vec![1], vec![0, 2], vec![1]];
        assert_eq!(bfs(&adjacency_test, 0, 0), Some(0));
    }

    // Test to check one-degree separation between two nodes
    #[test]
    fn check_one_degree() {
        let adjacency_test: Vec<Vec<usize>> = vec![vec![1], vec![0, 2], vec![1]];
        assert_eq!(bfs(&adjacency_test, 0, 1), Some(1));
    }

    // Test to check no connections between two nodes
    #[test]
    fn check_no_connections() {
        let adjacency_test: Vec<Vec<usize>> = vec![vec![1], vec![0], vec![3], vec![2]];
        assert_eq!(bfs(&adjacency_test, 0, 3), None);
    }

    // Test to count the number of valid connections in the graph
    #[test]
    fn check_valid_connections_count() {
        let adjacency_test: Vec<Vec<usize>> = vec![vec![1], vec![0, 2], vec![1, 3], vec![2]];
        let mut valid_connections = 0;
        let total_pairs = adjacency_test.len() * (adjacency_test.len() - 1);

        for i in 0..adjacency_test.len() {
            for j in 0..adjacency_test.len() {
                if i != j {
                    if bfs(&adjacency_test, i, j).is_some() {
                        valid_connections += 1;
                    }
                }
            }
        }

        assert_eq!(valid_connections, total_pairs);
    }
}
