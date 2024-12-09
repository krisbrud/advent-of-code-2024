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
    return (0 <= pos.0) && (pos.0 < rows) && (0 < pos.1) && (pos.1 < cols);
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

    let candidate_1 = (p1.0 - p2.0, p1.1 - p2.1);
    let candidate_2 = (p2.0 - p1.0, p2.1 - p1.1);

    vec![candidate_1, candidate_2]
        .iter()
        .filter(|candidate| is_within_bounds(**candidate, rows, cols))
        .map(|pos| (pos.0 as usize, pos.1 as usize))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    // Find the size of the board
    let rows = input.lines().collect_vec().len() as i32;
    let cols = input.lines().nth(0).map(|x| x.len())? as i32;

    // Parse all positions (row, col) and chars for nodes
    let nodes = Node::parse_all(input);

    // Find all potential antinodes
    let node_groups: Vec<(usize, usize)> = nodes.chunk_by(|a, b| a.c == b.c).flat_map(|group| {
        let group_vec = group.to_vec();
        let combinations: Vec<(Node, Node)> = group_vec
            .iter()
            .tuple_combinations::<(_, _)>()
            .map(move |(a, b)| (*a, *b))
            .collect();
        combinations
            .iter()
            .map(|(a, b)| antinode_positions(a.pos, b.pos, rows, cols))
            .collect::<Vec<_>>()
    }).collect();

    // Filter by antinodes within the size of the boards

    // Create hashset of all positions

    // Get count of elements in hashset

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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
