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
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new(edges: Vec<Edge>) -> Graph {
        let edges_by_first = edges.into_iter().into_group_map();
        Graph {
            edges: edges_by_first,
        }
    }
}

type ThreeClique = (String, String, String);

// Make it possible to have a unique representation of the clique by sorting the vertices
fn normalized(clique: ThreeClique) -> ThreeClique {
    let mut out = vec![clique.0, clique.1, clique.2];
    out.sort();
    (out[0].clone(), out[1].clone(), out[2].clone())
}

fn starts_with_t(s: &str) -> bool {
    s.chars().nth(0).expect("Should have first char") == 't'
}

fn clique_has_t(clique: &ThreeClique) -> bool {
    starts_with_t(&clique.0) || starts_with_t(&clique.1) | starts_with_t(&clique.2)
}

// From https://en.wikipedia.org/wiki/Bron–Kerbosch_algorithm
// algorithm BronKerbosch1(R, P, X) is
//     if P and X are both empty then
//         report R as a maximal clique
//     for each vertex v in P do
//         BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//         P := P \ {v}
//         X := X ⋃ {v}

struct Iteration {
    required: HashSet<String>,
    possible: HashSet<String>,
    excluded: HashSet<String>,
}

fn bron_kerbosch(possible: HashSet<String>, graph: &Graph) -> Vec<Vec<String>> {
    let mut iteration_stack: Vec<Iteration> = vec![];
    iteration_stack.push(Iteration {
        required: HashSet::new(),
        possible,
        excluded: HashSet::new(),
    });
    let mut cliques: Vec<Vec<String>> = vec![];

    while !iteration_stack.is_empty() {
        let iteration = iteration_stack.pop().expect("Should have iteratoin state");
        if iteration.possible.is_empty() && iteration.excluded.is_empty() {
            let clique = iteration.required.iter().cloned().collect_vec();
            cliques.push(clique);
        }

        let mut new_possible = iteration.possible.clone();
        let mut new_excluded = iteration.excluded.clone();
        for v in iteration.possible.iter() {
            let neighbors: HashSet<String> = graph
                .edges
                .get(v)
                .expect("Should have neighbors")
                .iter()
                .cloned()
                .collect();
            let required_union_v: HashSet<String> = iteration
                .required
                .union(&HashSet::from([v.clone()]))
                .cloned()
                .collect::<HashSet<String>>();
            let possible_intersection_neigh_v: HashSet<String> = new_possible
                .intersection(&HashSet::from(neighbors.clone()))
                .cloned()
                .collect::<HashSet<String>>();
            let excluded_intersection_neigh_v: HashSet<String> = new_excluded
                .intersection(&HashSet::from(neighbors))
                .cloned()
                .collect::<HashSet<String>>();

            // let required_union_v: HashSet<String> = required.clone().union(&HashSet::from([v.clone()])).cloned().collect::<HashSet<String>>();
            iteration_stack.push(Iteration {
                required: required_union_v,
                possible: possible_intersection_neigh_v,
                excluded: excluded_intersection_neigh_v,
            });
            // bron_kerbosch(
            //     required_union_v,
            //     possible_intersection_v,
            //     excluded_intersection_v,
            //     cliques,
            //     graph,
            // );
            new_possible.remove(v);
            new_excluded.insert(v.clone());
        }
    }
    cliques
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

    let graph = Graph::new(edges);
    // dbg!(edges.len());
    // dbg!(unique_vertices.len());
    let mut cliques: HashSet<ThreeClique> = HashSet::new();
    for u in unique_vertices {
        let neighs = graph.edges.get(&u).expect("Should find neighbors");
        for (v, w) in neighs.iter().tuple_combinations() {
            if graph
                .edges
                .get(v)
                .expect("Should find inner neighbors")
                .contains(w)
            {
                cliques.insert(normalized((u.clone(), v.clone(), w.clone())));
            }
        }
    }

    // Find all 3-cliques by brute-force
    // Start in each vertex, for each adjacent neighbor explore it and its neighbors
    // If the start vector is found in 3 steps, we have found a clique
    let relevant_cliques = cliques
        .into_iter()
        .filter(|clique| clique_has_t(clique))
        .collect_vec();

    // Find all the sets of three inter-connected computers. How many contain at least one computer with a name that starts with t?
    Some(
        relevant_cliques
            .len()
            .try_into()
            .expect("Should convert u32 to usize"),
    )
    // None
}

pub fn part_two(input: &str) -> Option<String> {
    let edges_single_direction = parse_edges(input).expect("Should parse edges");
    let edges_opposite_direction = edges_single_direction
        .iter()
        .map(|edge| swapped_direction_edge(edge.clone()))
        .collect_vec();

    let edges = add_vecs(edges_single_direction, edges_opposite_direction);

    let unique_vertices: HashSet<String> = edges
        .iter()
        .flat_map(|(first, second)| vec![first.clone(), second.clone()])
        .collect();

    let graph = Graph::new(edges);

    println!("Before bron kerbosch");
    let cliques = bron_kerbosch(unique_vertices, &graph);
    println!("cliques: {}", cliques.len());

    let mut biggest_clique = cliques
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .expect("Should have max!").clone();

    biggest_clique.sort();

    let output = biggest_clique.iter().join(",");
    Some(output)
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
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
