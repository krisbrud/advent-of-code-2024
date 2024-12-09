use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Clone, Copy)]
struct Node {
    pos: (usize, usize),
    c: char,
}

impl Node {
    fn parse_all(input: &str) -> Vec<Node> {
        input
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, c)| Node { pos: (row, col), c })
            })
            .filter(|node| node.c != '.') // Placeholder for empty
            .collect()
    }
}

fn is_within_bounds(pos: (i32, i32), rows: i32, cols: i32) -> bool {
    return (0 <= pos.0) && (pos.0 < rows) && (0 <= pos.1) && (pos.1 < cols);
}

fn pos_as_i32(pos: (usize, usize)) -> (i32, i32) {
    (pos.0 as i32, pos.1 as i32)
}

fn antinode_positions_part_1(
    pos1: (usize, usize),
    pos2: (usize, usize),
    rows: i32,
    cols: i32,
) -> Vec<(usize, usize)> {
    let p1 = pos_as_i32(pos1);
    let p2 = pos_as_i32(pos2);

    // x_1 = 5, x_2 = 3 => x_anti_1 = 5 - 2*(5-3)
    let candidate_1 = (p1.0 - 2 * (p1.0 - p2.0), p1.1 - 2 * (p1.1 - p2.1));
    let candidate_2 = (p2.0 - 2 * (p2.0 - p1.0), p2.1 - 2 * (p2.1 - p1.1));

    vec![candidate_1, candidate_2]
        .iter()
        .filter(|candidate| is_within_bounds(**candidate, rows, cols))
        .map(|pos| (pos.0 as usize, pos.1 as usize))
        .collect()
}

fn add(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn minus(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

fn multiply(vec: (i32, i32), scalar: i32) -> (i32, i32) {
    (vec.0 * scalar, vec.1 * scalar)
}

fn antinode_positions_part_2(
    pos1: (usize, usize),
    pos2: (usize, usize),
    rows: i32,
    cols: i32,
) -> Vec<(usize, usize)> {
    let p1 = pos_as_i32(pos1);
    let p2 = pos_as_i32(pos2);

    // x_1 = 5, x_2 = 3 => x_anti_1 = 5 - 2*(5-3)
    // x_1 = 5, x_2 = 3 => diff = 2
    //                  => add diff from 1 and subtract from 2
    //                     BUT - we also want the nodes themselves to be antinodes, so we swap them here
    let diff = ((p1.0 - p2.0), (p1.1 - p2.1));

    let mut antinode_positions: Vec<(usize, usize)> = vec![];

    for i in 1.. {
        let scaled_diff = multiply(diff, i);
        let candidate = add(p2, scaled_diff);

        if is_within_bounds(candidate, rows, cols) {
            antinode_positions.push((candidate.0 as usize, candidate.1 as usize));
        } else {
            break
        }
    }

    for i in 1.. {
        let scaled_diff = multiply(diff, -i);
        let candidate = add(p1, scaled_diff);

        if is_within_bounds(candidate, rows, cols) {
            antinode_positions.push((candidate.0 as usize, candidate.1 as usize));
        } else {
            break
        }
    }

    antinode_positions
}

pub fn part_one(input: &str) -> Option<u32> {
    // Find the size of the board
    let rows = input.lines().collect_vec().len() as i32;
    let cols = input.lines().nth(0).map(|x| x.len())? as i32;

    // Parse all positions (row, col) and chars for nodes
    let nodes = Node::parse_all(input);

    // Find all potential antinodes
    let antinodes: Vec<(usize, usize)> = nodes
        .iter()
        .into_group_map_by(|x| x.c)
        .values()
        .flat_map(|group| -> Vec<(usize, usize)> {
            let combinations: Vec<(Node, Node)> = group
                .iter()
                .tuple_combinations::<(_, _)>()
                .map(|(a, b)| (**a, **b))
                .collect();
            let group_antinode_positions = combinations
                .iter()
                .flat_map(|(a, b)| antinode_positions_part_1(a.pos, b.pos, rows, cols))
                .collect::<Vec<(usize, usize)>>();
            group_antinode_positions
        })
        .collect();

    // Create hashset of all positions
    let positions: HashSet<(usize, usize)> = antinodes.into_iter().collect();

    // Get count of elements in hashset
    let count = positions.into_iter().count();

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Find the size of the board
    let rows = input.lines().collect_vec().len() as i32;
    let cols = input.lines().nth(0).map(|x| x.len())? as i32;

    // Parse all positions (row, col) and chars for nodes
    let nodes = Node::parse_all(input);

    // Find all potential antinodes
    let antinodes: Vec<(usize, usize)> = nodes
        .iter()
        .into_group_map_by(|x| x.c)
        .values()
        .flat_map(|group| -> Vec<(usize, usize)> {
            let combinations: Vec<(Node, Node)> = group
                .iter()
                .tuple_combinations::<(_, _)>()
                .map(|(a, b)| (**a, **b))
                .collect();
            let group_antinode_positions = combinations
                .iter()
                .flat_map(|(a, b)| antinode_positions_part_2(a.pos, b.pos, rows, cols))
                .collect::<Vec<(usize, usize)>>();
            group_antinode_positions
        })
        .collect();

    // Create hashset of all positions
    let positions: HashSet<(usize, usize)> = antinodes.into_iter().collect();

    // Get count of elements in hashset
    let count = positions.into_iter().count();

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
