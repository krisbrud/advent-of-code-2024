use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(20);

type Coord = (usize, usize);

struct Board {
    tiles: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn new(s: &str) -> Option<Board> {
        let plants: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let rows = s.lines().collect_vec().len();
        let cols = s.lines().nth(0).map(|x| x.len())?;

        Some(Board {
            tiles: plants,
            rows,
            cols,
        })
    }

    // Assume bounds check has already been done
    fn get(&self, coordinate: &Coord) -> char {
        self.tiles[coordinate.0][coordinate.1]
    }
}

fn find_needle_or_fail(board: &Board, needle: char) -> Coord {
    for (i, row) in board.tiles.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == needle {
                return (i, j);
            }
        }
    }

    panic!("Couldn't find tile!");
}

enum Direction {
    North,
    East,
    South,
    West,
}

// Coordinates in format (y, x)
fn coord_in_direction(board: &Board, coord: Coord, direction: Direction) -> Option<Coord> {
    let coord = match direction {
        Direction::North => Some((coord.0.checked_sub(1)?, coord.1)),
        Direction::South => Some((coord.0 + 1, coord.1)),
        Direction::East => Some((coord.0, coord.1 + 1)),
        Direction::West => Some((coord.0, coord.1.checked_sub(1)?)),
    }?;

    if coord.0 < board.rows && coord.1 < board.cols {
        Some(coord)
    } else {
        None
    }
}

fn adjacent_coords(board: &Board, coord: Coord) -> Vec<Coord> {
    vec![
        coord_in_direction(board, coord, Direction::North),
        coord_in_direction(board, coord, Direction::East),
        coord_in_direction(board, coord, Direction::South),
        coord_in_direction(board, coord, Direction::West),
    ]
    .iter()
    .filter_map(|c| *c)
    .collect()
}

const START: char = 'S';
const GOAL: char = 'E';
const WALL: char = '#';

fn baseline(board: &Board, start_coord: Coord, goal_coord: Coord) -> HashMap<Coord, u32> {
    let mut distances: HashMap<Coord, u32> = HashMap::new();

    distances.insert(start_coord, 0);

    let mut current = start_coord;
    let mut cost: u32 = 0;
    while current != goal_coord {
        let adjacent_coords = adjacent_coords(board, current);
        let next = adjacent_coords
            .iter()
            .filter(|coord| (board.get(coord) != WALL) && !distances.contains_key(coord))
            .next()
            .expect("Should find next"); // Should always just be a single value
        cost += 1;

        distances.insert(*next, cost);
        current = *next;
    }

    distances
}

// Returns the saving of the cheat if it exists
fn check_for_cheat(board: &Board, distances: &HashMap<Coord, u32>, coord: Coord) -> Option<u32> {
    // Note: When doing coord_in_direction, we can safely return false for the option early, since no border coord can be part of a cheat
    let north_coord = coord_in_direction(board, coord, Direction::North)?;
    let south_coord = coord_in_direction(board, coord, Direction::South)?;
    let east_coord = coord_in_direction(board, coord, Direction::East)?;
    let west_coord = coord_in_direction(board, coord, Direction::West)?;

    if let Some(cheat) = check_coords_for_cheat(board, distances, north_coord, south_coord) {
        return Some(cheat);
    }

    if let Some(cheat) = check_coords_for_cheat(board, distances, west_coord, east_coord) {
        return Some(cheat);
    }

    None
}

fn check_coords_for_cheat(
    board: &Board,
    distances: &HashMap<(usize, usize), u32>,
    first: (usize, usize),
    second: (usize, usize),
) -> Option<u32> {
    let first_tile = board.get(&first);
    let second_tile = board.get(&second);
    if first_tile != WALL && second_tile != WALL {
        if let (Some(first_cost), Some(second_cost)) =
            (distances.get(&first), distances.get(&second))
        {
            // Since a cheat involves spending one step inside the wall, we
            let diff: i32 = first_cost.abs_diff(*second_cost).try_into().ok()?;
            let savings_potential = diff - 2;
            if savings_potential > 0 {
                return Some(savings_potential.try_into().ok()?);
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let board = Board::new(input)?;
    // Note: There are no branches in the track
    // Traverse and keep track of the cost per tile
    let start_coord = find_needle_or_fail(&board, START);
    let goal_coord = find_needle_or_fail(&board, GOAL);

    println!("Finding distances!");
    let distances = baseline(&board, start_coord, goal_coord);

    println!("Found distances, checking cheats!");
    let cheat_wall_candidate_coords: Vec<Coord> = board
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| **tile == WALL)
                .map(move |(j, _)| (i, j))
        })
        .collect();

    println!("Found cheat_wall_candidate_coords!");
    // Then, for each wall in the board, check
    // - left + right
    // - top + bottom
    // For each wall, only one of these can be a valid cheat.
    let cheats: Vec<u32> = cheat_wall_candidate_coords
        .iter()
        .map(|coord| check_for_cheat(&board, &distances, *coord))
        .filter_map(|x| x)
        .collect();

    let cheat_over_threshold_count = cheats.iter().filter(|cheat| **cheat >= 100).count();

    Some(cheat_over_threshold_count)
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
        assert_eq!(result, Some(0));
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
