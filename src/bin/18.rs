use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(18);

type Coord = (usize, usize);

struct Board {
    occupied: HashSet<Coord>,
    rows: usize,
    cols: usize,
}

fn parse_coord(s: &str) -> Option<Coord> {
    let first: usize = s.split(",").nth(0)?.parse().ok()?;
    let second: usize = s.split(",").nth(1)?.parse().ok()?;

    Some((first, second))
}

fn neighbor_coordinates(coord: &Coord, rows: usize, cols: usize) -> Vec<Coord> {
    let mut neighbors: Vec<Coord> = vec![];

    if coord.0 > 0 {
        neighbors.push((coord.0 - 1, coord.1));
    }

    if coord.0 < cols - 1 {
        neighbors.push((coord.0 + 1, coord.1));
    }

    if coord.1 > 0 {
        neighbors.push((coord.0, coord.1 - 1));
    }

    if coord.1 < rows - 1 {
        neighbors.push((coord.0, coord.1 + 1));
    }

    neighbors
}

impl Board {
    fn new(s: Vec<&str>, is_test: bool) -> Option<Board> {
        let rows: usize = if is_test { 7 } else { 71 };
        let cols: usize = rows;
        let tiles: HashSet<Coord> = s
            .iter()
            .map(|line| parse_coord(&line))
            .collect::<Option<HashSet<_>>>()?;

        Some(Board {
            occupied: tiles,
            rows,
            cols,
        })
    }
}

fn backtrack_steps(predecessors: &HashMap<Coord, Coord>, start: Coord, current: Coord) -> u32 {
    let predecessor = *predecessors
        .get(&current)
        .expect("Should have predecessor!");

    if predecessor == start {
        return 1;
    } else {
        return 1 + backtrack_steps(predecessors, start, predecessor);
    }
}

// Assume going from top left to bottom right
fn bfs(board: &Board) -> Option<u32> {
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut predecessors: HashMap<Coord, Coord> = HashMap::new();
    let mut queue: VecDeque<Coord> = VecDeque::new();
    let start: Coord = (0, 0);
    let goal: Coord = (board.rows - 1, board.cols - 1);
    queue.push_back(start);
    visited.insert(start);

    while let Some(coord) = queue.pop_front() {
        if coord == goal {
            return Some(backtrack_steps(&predecessors, start, coord));
        }

        let neighbors = neighbor_coordinates(&coord, board.rows, board.cols)
            .into_iter()
            .filter(|candidate| !visited.contains(candidate))
            .filter(|candidate| !board.occupied.contains(candidate))
            .collect_vec();

        for neigh in neighbors {
            visited.insert(neigh.clone());
            predecessors.insert(neigh.clone(), coord);
            queue.push_back(neigh.clone());
        }
    }

    None // No solution
}

pub fn part_one(input: &str) -> Option<u32> {
    let is_test = input.lines().count() == 25;
    let bytes_to_take: usize = if is_test { 12 } else { 1024 };

    let lines: Vec<&str> = input.lines().take(bytes_to_take).collect();
    let board = Board::new(lines, is_test).expect("Should parse board!");

    let steps = bfs(&board).expect("Should find bfs solution");

    Some(steps)
}

pub fn part_two(input: &str) -> Option<String> {
    let num_lines = input.lines().count();
    let is_test = num_lines == 25;

    // Should probably have used binary search here, but let's just brute force it to catch up on the other puzzles
    for i in 1..num_lines {
        let bytes_to_take = num_lines - i;
        let lines: Vec<&str> = input.lines().take(bytes_to_take).collect();
        let board = Board::new(lines, is_test).expect("Should parse board!");
        let maybe_steps = bfs(&board);

        if maybe_steps.is_some() {
            let first_blocking_coord = input
                .lines()
                .nth(bytes_to_take)
                .expect("Should find first blocking coord");
            return Some(first_blocking_coord.to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
