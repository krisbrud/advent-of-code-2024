use itertools::Itertools;
use std::{collections::HashMap, env::var};

advent_of_code::solution!(6);

#[derive(Clone, Copy)]
struct BoardSize {
    rows: usize,
    cols: usize,
}

#[derive(Clone, Debug, PartialEq)]
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

fn pos_in_direction(
    pos: (usize, usize),
    direction: &Direction,
    size: &BoardSize,
) -> Option<(usize, usize)> {
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
            if pos.1 > 0 {
                Some((pos.0, pos.1 - 1))
            } else {
                None
            }
        }
        Direction::East => {
            if pos.1 < size.cols - 1 {
                Some((pos.0, pos.1 + 1))
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Debug)]
enum Tile {
    Unvisited,
    Obstruction,
    Guard(Direction),
    VisitedPart1,            // Direction is where the player left the last time
    VisitedPart2(Direction), // Direction is where the player left the last time
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

#[derive(Clone)]
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

    fn print(board: &Board) {
        for row in 0..board.size.rows {
            for col in 0..board.size.cols {
                let tile: &Tile = board.tiles.get(&(row, col)).unwrap();
                let char = match tile {
                    Tile::Unvisited => '.',
                    Tile::Obstruction => '#',
                    Tile::Guard(direction) => match direction {
                        Direction::North => 'N',
                        Direction::East => 'E',
                        Direction::South => 'S',
                        Direction::West => 'W',
                    },
                    Tile::VisitedPart1 => 'X',
                    Tile::VisitedPart2(direction) => match direction {
                        Direction::North => '^',
                        Direction::East => '>',
                        Direction::South => 'V',
                        Direction::West => '<',
                    },
                };
                print!("{}", char);
            }
            print!("\n")
        }
    }
}

fn step_part_1(tiles: &mut HashMap<(usize, usize), Tile>, size: &BoardSize) -> bool {
    // let (guard_pos, guard_tile) = tiles.iter().find(|(pos, tile)| {
    let (guard_pos, guard_tile) = tiles
        .iter()
        .find(|(_, tile)| matches!(tile, Tile::Guard(_)))
        .map(|(pos, tile)| (*pos, tile.clone()))
        .unwrap();

    tiles.insert(guard_pos, Tile::VisitedPart1);

    let guard_direction = match guard_tile.clone() {
        Tile::Guard(direction) => direction,
        _ => panic!("Guard tile should always be guard"),
    };

    if let Some(pos_in_front) = pos_in_direction(guard_pos, &guard_direction, size) {
        if let Some(tile_in_front) = tiles.get(&pos_in_front) {
            match tile_in_front {
                Tile::Unvisited | Tile::VisitedPart1 => {
                    // Set tile in front as guard with same direction
                    tiles.insert(pos_in_front, guard_tile);
                    return false;
                }
                Tile::Obstruction => {
                    let turned_direction = guard_direction.turn_right();
                    if let Some(turned_pos) = pos_in_direction(guard_pos, &turned_direction, &size)
                    {
                        tiles.insert(turned_pos, Tile::Guard(turned_direction.clone()));
                        return false;
                    } else {
                        // Outside of board
                        return true;
                    }
                }
                Tile::Guard(_) => {
                    panic!("should not have more than one guard")
                }
                Tile::VisitedPart2(_) => panic!("Should not find these"),
            }
        } else {
            panic!("should always find tile")
        }
    } else {
        println!("Outside of board!");
        true
    }
}

// fn loops(tiles: &mut HashMap<(usize, usize), Tile>, size: &BoardSize) -> u32 {
//     // let (guard_pos, guard_tile) = tiles.iter().find(|(pos, tile)| {
//     let (guard_pos, guard_tile) = tiles
//         .iter()
//         .find(|(_, tile)| matches!(tile, Tile::Guard(_)))
//         .map(|(pos, tile)| (*pos, tile.clone()))
//         .unwrap();

//     // tiles.insert(guard_pos, Tile::VisitedPart2());

//     let guard_direction = match guard_tile.clone() {
//         Tile::Guard(direction) => direction,
//         _ => panic!("Guard tile should always be guard"),
//     };

//     // let non_branching_loops: u32 = if let Some(pos_in_front) =
//     if let Some(pos_in_front) = pos_in_direction(guard_pos, &guard_direction, size) {
//         tiles.insert(guard_pos, Tile::VisitedPart2(guard_direction.clone()));
//         if let Some(tile_in_front) = tiles.get(&pos_in_front) {
//             match tile_in_front {
//                 Tile::Unvisited => {
//                     // Set tile in front as guard with same direction
//                     tiles.insert(pos_in_front, guard_tile);
//                     loops(tiles, size)
//                 }
//                 Tile::VisitedPart2(prev_direction) => {
//                     if guard_direction == *prev_direction {
//                         // Found a loop!
//                         return 1;
//                     }
//                     tiles.insert(pos_in_front, guard_tile);
//                     loops(tiles, size)
//                 }
//                 Tile::Obstruction => {
//                     let turned_direction = guard_direction.turn_right();
//                     if let Some(turned_pos) = pos_in_direction(guard_pos, &turned_direction, &size)
//                     {
//                         tiles.insert(turned_pos, Tile::Guard(turned_direction.clone()));
//                         loops(tiles, size)
//                     } else {
//                         // Outside of board
//                         0
//                     }
//                 }
//                 Tile::Guard(_) => {
//                     panic!("should not have more than one guard")
//                 }
//                 Tile::VisitedPart1 => panic!("Shouldn't find part1 visited"),
//             }
//         } else {
//             panic!("should always find tile")
//         }
//     } else {
//         0
//     }
// }

fn loops(tiles: &mut HashMap<(usize, usize), Tile>, size: &BoardSize) -> u32 {
    // let (guard_pos, guard_tile) = tiles.iter().find(|(pos, tile)| {
    let (guard_pos, guard_tile) = tiles
        .iter()
        .find(|(_, tile)| matches!(tile, Tile::Guard(_)))
        .map(|(pos, tile)| (*pos, tile.clone()))
        .unwrap();

    // tiles.insert(guard_pos, Tile::VisitedPart2());

    let guard_direction = match guard_tile.clone() {
        Tile::Guard(direction) => direction,
        _ => panic!("Guard tile should always be guard"),
    };

    // let non_branching_loops: u32 = if let Some(pos_in_front) =
    if let Some(pos_in_front) =
        pos_in_direction(guard_pos, &guard_direction, size)
    {
        tiles.insert(guard_pos, Tile::VisitedPart2(guard_direction.clone()));
        if let Some(tile_in_front) = tiles.get(&pos_in_front) {
            match tile_in_front {
                Tile::Unvisited => {
                    // Set tile in front as guard with same direction
                    tiles.insert(pos_in_front, guard_tile);
                    loops(tiles, size)
                }
                Tile::VisitedPart2(prev_direction) => {
                    if guard_direction == *prev_direction {
                        // Found a loop!
                        return 1;
                    }
                    tiles.insert(pos_in_front, guard_tile);
                    loops(tiles, size)
                }
                Tile::Obstruction => {
                    let turned_direction = guard_direction.turn_right();
                    if let Some(turned_next_pos) =
                        pos_in_direction(guard_pos, &turned_direction, &size)
                    {
                        if let Some(turned_next_next_pos) =
                            pos_in_direction(turned_next_pos, &turned_direction, size)
                        {
                            if let Some(turned_next_next_tile) = tiles.get(&turned_next_next_pos) {
                                match turned_next_next_tile {
                                    Tile::Obstruction => {
                                        if let Some(turned_next_tile) = tiles.get(&turned_next_pos)
                                        {
                                            match turned_next_tile {
                                                Tile::VisitedPart2(next_direction)
                                                    if turned_direction.turn_right()
                                                        == *next_direction =>
                                                {
                                                    // Loop!
                                                    println!("Obstruction loop found!");
                                                    return 1;
                                                }
                                                _ => (),
                                            }
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }

                        // tiles.insert(turned_next_pos, Tile::Guard(guard_direction.clone()));
                        tiles.insert(turned_next_pos, Tile::Guard(turned_direction.clone()));
                        loops(tiles, size)
                    } else {
                        // Outside of board
                        0
                    }
                }
                Tile::Guard(_) => {
                    panic!("should not have more than one guard")
                }
                Tile::VisitedPart1 => panic!("Shouldn't find part1 visited"),
            }
        } else {
            panic!("should always find tile")
        }
    } else {
        0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::parse(input)?;
    let size = board.size.clone();
    let mut tiles = board.tiles.clone();

    let mut finished = false;
    let max_steps = 5000;

    let mut steps = 0;
    while !finished && steps < max_steps {
        // println!("{}", steps);
        // let cloned_tiles = tiles.clone();
        // Board::print(&Board { tiles: cloned_tiles, size });
        finished = step_part_1(&mut tiles, &size);
        steps += 1;
    }

    let count: u32 = tiles
        .clone()
        .iter()
        .filter(|(_, tile)| matches!(tile, Tile::VisitedPart1))
        .count()
        .try_into()
        .unwrap();

    Some(count)
}

// pub fn part_two(input: &str) -> Option<u32> {
//     let board = Board::parse(input)?;
//     let size = board.size.clone();
//     let mut tiles = board.tiles.clone();

//     let loop_count = loops(&mut tiles, &size, false);

//     Some(loop_count)
// }

pub fn part_two(input: &str) -> Option<u32> {
    let board = Board::parse(input)?;
    let size = board.size.clone();
    let mut tiles = board.tiles.clone();

    let mut loop_count = 0;

    let variants = board.tiles.iter().filter(|(_, tile)| {
        match tile {
            Tile::Unvisited => true,
            _ => false
        }
    }).map(|(pos, _)| pos);


    for variant_pos in variants {
        let mut tile_variant = tiles.clone();
        tile_variant.insert(*variant_pos, Tile::Obstruction);

        loop_count += loops(&mut tile_variant, &size);
    }

    Some(loop_count)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_small_loop() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(1));
    }
}
