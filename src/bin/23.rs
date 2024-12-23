use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

type Edge = (String, String);

fn swapped_direction_edge(edge: Edge) -> Edge {
    (edge.1, edge.0)
}

fn parse_edges(input: &str) -> Option<Vec<Edge>> {
    input
        .lines()
        .map(|line| {
            if let (Some(first), Some(second)) = (line.split("-").nth(0), line.split("-").nth(1)) {
                Some((first.to_string(), second.to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn add_vecs(first: Vec<Edge>, second: Vec<Edge>) -> Vec<Edge> {
    let mut out = first.clone();
    out.append(&mut second.clone());
    out
}

struct Graph {
    edges: HashMap<String, Vec<String>>
}

type ThreeClique = (String, String, String);

// Make it possible to have a unique representation of the clique by sorting the vertices
fn normalized(clique: ThreeClique) -> ThreeClique {
    let mut out = vec![clique.0, clique.1, clique.2];
    out.sort();
    (out[0].clone(), out[1].clone(), out[2].clone())
}

fn traverse_n_steps(graph: &Graph, from: &str, steps: usize) -> Vec<Vec<String>> {
    // Base case: steps = 1 or 0?

    vec![]
}

fn find_three_cliques(graph: &Graph, from: &str) -> Vec<ThreeClique> {
    // Find all 3-cliques by brute-force
    // Start in each vertex, for each adjacent neighbor explore it and its neighbors
    // If the start vector is found in 3 steps, we have found a clique
    // let mut out: Vec<Vec<String>>

    for depth in 0..3 {

    }

    vec![]
}


pub fn part_one(input: &str) -> Option<u32> {
    // Idea: Create HashMap from beginning to list/hashset of adjacent vertices
    let edges_single_direction = parse_edges(input).expect("Should parse edges");
    let edges_opposite_direction = edges_single_direction
        .iter()
        .map(|edge| swapped_direction_edge(edge.clone()))
        .collect_vec();

    let edges = add_vecs(edges_single_direction, edges_opposite_direction);

    // Find all unique vertices
    let unique_vertices: HashSet<String> = edges
        .iter()
        .flat_map(|(first, second)| vec![first.clone(), second.clone()])
        .collect();

    dbg!(edges.len());
    dbg!(unique_vertices.len());

    // Find all 3-cliques by brute-force
    // Start in each vertex, for each adjacent neighbor explore it and its neighbors
    // If the start vector is found in 3 steps, we have found a clique

    // Find all the sets of three inter-connected computers. How many contain at least one computer with a name that starts with t?
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
