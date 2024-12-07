use itertools::Itertools;
use std::{
    borrow::{Borrow, BorrowMut},
    collections::{hash_map::Entry, HashMap},
};

advent_of_code::solution!(6);

struct BoardSize {
    rows: usize,
    cols: usize,
}

#[derive(Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Self::East,
            Direction::East => Self::South,
            Direction::South => Self::West,
            Direction::West => Self::North,
        }
    }
}

fn pos_in_direction(pos: (usize, usize), direction: Direction, size: BoardSize) -> Option<(usize, usize)> {
    match direction {
        Direction::North => {
            if pos.0 > 0 {
                Some((pos.0 - 1, pos.1))
            } else {
                None
            }
        }
        Direction::South => {
            if pos.0 < size.rows - 1 {
                Some((pos.0 + 1, pos.1))
            } else {
                None
            }
        }
        Direction::West => {
            if pos.0 < size.rows - 1 {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
        Direction::East => {
            if pos.0 < size.rows - 1 {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            }
        }
    }
}

#[derive(Clone)]
enum Tile {
    Unvisited,
    Obstruction,
    Guard(Direction),
    Visited,
}

impl Tile {
    fn parse(char: char) -> Option<Tile> {
        match char {
            '.' => Some(Self::Unvisited),
            '#' => Some(Self::Obstruction),
            '^' => Some(Self::Guard(Direction::North)),
            '>' => Some(Self::Guard(Direction::East)),
            'v' => Some(Self::Guard(Direction::South)),
            '<' => Some(Self::Guard(Direction::West)),
            _ => None,
        }
    }
}

struct Board {
    tiles: HashMap<(usize, usize), Tile>,
    size: BoardSize,
}

// enum StepResult {
//     Unfinished { board: Board, next_pose: Pose },
//     Finished { board: Board },
// }

impl Board {
    fn parse(s: &str) -> Option<Board> {
        let rows = s.lines().collect_vec().len();
        let cols = s.lines().nth(0).map(|x| x.len())?;

        let tiles = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.char_indices().map(move |(col, char)| {
                    let pos: (usize, usize) = (row, col);
                    if let Some(tile) = Tile::parse(char) {
                        Some((pos, tile))
                    } else {
                        None
                    }
                })
            })
            .collect::<Option<HashMap<(usize, usize), Tile>>>()?;

        Some(Board {
            tiles,
            size: BoardSize { rows, cols },
        })
    }
}

fn step(tiles: &mut HashMap<(usize, usize), Tile>, size: &BoardSize) -> bool {
    // let (guard_pos, guard_tile) = tiles.iter().find(|(pos, tile)| {
    let (guard_pos, guard_tile) = tiles
        .iter()
        .find(|(_, tile)| matches!(tile, Tile::Guard(_)))
        .map(|(pos, tile)| (*pos, tile.clone()))
        .unwrap();

    tiles.insert(guard_pos, Tile::Visited);

    let pos_in_front = match guard_tile {
        Tile::Guard(Direction::North) => {
            if guard_pos.0 > 0 {
                (guard_pos.0 - 1, guard_pos.1)
            } else {
                return false;
            }
        }
        Tile::Guard(Direction::South) => {
            if guard_pos.0 < size.rows - 1 {
                (guard_pos.0 + 1, guard_pos.1)
            } else {
                return false;
            }
        }
        Tile::Guard(Direction::West) => {
            if guard_pos.0 < size.rows - 1 {
                (guard_pos.0, guard_pos.1 - 1)
            } else {
                return false;
            }
        }
        Tile::Guard(Direction::East) => {
            if guard_pos.0 < size.rows - 1 {
                (guard_pos.0, guard_pos.1 + 1)
            } else {
                return false;
            }
        }
        _ => {
            println!("Shouldn't reach here!");
            return false;
        }
    };

    if let Some(tile_in_front) = tiles.get(&pos_in_front) {
        match tile_in_front {
            Tile::Unvisited | Tile::Visited => {
                // Set tile in front as guard with same direction
                tiles.insert(pos_in_front, guard_tile);
            }
            Tile::Obstruction => {
                // Turn right
            }
            Tile::Guard(_) => {
                panic!("should not have more guards")
            }
        }
    } else {
        panic!("should always find tile")
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::parse(input)?;
    let size = board.size;
    let mut tiles = board.tiles;

    let mut finished = false;
    let max_steps = 100000;

    let mut steps = 0;
    while !finished && steps < max_steps {
        finished = step(&mut tiles, &size);
        steps += 1;
    }

    let count: u32 = tiles
        .clone()
        .iter()
        .filter(|(_, tile)| matches!(tile, Tile::Visited))
        .count()
        .try_into()
        .unwrap();

    Some(count)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
