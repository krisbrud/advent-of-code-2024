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

fn antinode_positions(
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

pub fn part_one(input: &str) -> Option<u32> {
    println!("{}", input);

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
                .flat_map(|(a, b)| antinode_positions(a.pos, b.pos, rows, cols))
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
    None
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
        assert_eq!(result, None);
    }
}
