use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(6);

#[derive(Clone, Copy)]
struct BoardSize {
    rows: usize,
    cols: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

type Coord = (usize, usize);

fn pos_in_direction(pos: Coord, direction: &Direction, size: &BoardSize) -> Option<(usize, usize)> {
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    tiles: HashMap<Coord, Tile>,
    size: BoardSize,
}

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
    let (guard_pos, guard_tile) = find_guard(tiles);

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
                    tiles.insert(guard_pos, Tile::Guard(turned_direction));
                    return false;
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

fn find_guard(tiles: &mut HashMap<(usize, usize), Tile>) -> ((usize, usize), Tile) {
    let (guard_pos, guard_tile) = tiles
        .iter()
        .find(|(_, tile)| matches!(tile, Tile::Guard(_)))
        .map(|(pos, tile)| (*pos, tile.clone()))
        .expect("should find guard!");
    (guard_pos, guard_tile)
}

struct StepResult {
    found_loop: bool,
    finished: bool,
}

#[derive(PartialEq, Eq, Hash)]
struct SeenTile {
    coord: Coord,
    tile: Tile,
}

fn step_part_2(
    tiles: &mut HashMap<Coord, Tile>,
    seen: &mut HashSet<SeenTile>,
    size: &BoardSize,
) -> StepResult {
    // let (guard_pos, guard_tile) = tiles.iter().find(|(pos, tile)| {
    let (guard_pos, guard_tile) = tiles
        .iter()
        .find(|(_, tile)| matches!(tile, Tile::Guard(_)))
        .map(|(pos, tile)| (*pos, tile.clone()))
        .expect("part 2: should find guard!");

    let seen_tile = SeenTile {
        coord: guard_pos,
        tile: guard_tile.clone(),
    };
    if seen.contains(&seen_tile) {
        // Found loop!
        let found_loop = true;
        let finished = false;
        return StepResult {
            found_loop,
            finished,
        };
    } else {
        seen.insert(seen_tile);
    }

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

                    return StepResult {
                        found_loop: false,
                        finished: false,
                    };
                }
                Tile::Obstruction => {
                    let turned_direction = guard_direction.turn_right();
                    tiles.insert(guard_pos, Tile::Guard(turned_direction));
                    return StepResult {
                        found_loop: false,
                        finished: false,
                    };
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
        // println!("Outside of board!");
        return StepResult {
            found_loop: false,
            finished: true,
        };
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::parse(input)?;
    let size = board.size.clone();
    let mut tiles = board.tiles.clone();

    let mut finished = false;
    let max_steps = 10000;

    let mut steps = 0;
    while !finished && steps < max_steps {
        // println!("{}", steps);
        // let cloned_tiles = tiles.clone();
        // Board::print(&Board { tiles: cloned_tiles, size });
        // finished = step_part_1(&mut tiles, &size);
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

fn normal_path_coords(input: &str) -> Option<Vec<Coord>> {
    let board = Board::parse(input)?;
    let size = board.size.clone();
    let mut tiles = board.tiles.clone();

    let mut finished = false;
    let max_steps = 15000;

    let mut steps = 0;
    while !finished && steps < max_steps {
        // println!("{}", steps);
        // let cloned_tiles = tiles.clone();
        // Board::print(&Board { tiles: cloned_tiles, size });
        // finished = step_part_1(&mut tiles, &size);
        finished = step_part_1(&mut tiles, &size);
        steps += 1;
    }

    let coords: Vec<Coord> = tiles
        .clone()
        .iter()
        .filter(|(_, tile)| matches!(tile, Tile::VisitedPart1))
        .map(|(coord, _)| *coord)
        .collect();

    Some(coords)
}

pub fn part_two(input: &str) -> Option<u32> {
    let board = Board::parse(input)?;
    let size = board.size.clone();
    let mut tiles = board.tiles.clone();

    let (guard_pos, _) = find_guard(&mut tiles);
    let path_coords =normal_path_coords(input)?;
    // dbg!(path_coords.len());
    let variants =
        path_coords.into_iter()
        .filter(|coord| *coord != guard_pos)
        .collect_vec();
    // dbg!(variants.len());

    let loop_count = variants
        .into_iter()
        .filter(|variant_coord| variant_has_loop(size, &tiles, *variant_coord))
        .count()
        .try_into()
        .ok()?;

    Some(loop_count)
}

fn variant_has_loop(
    size: BoardSize,
    tiles: &HashMap<(usize, usize), Tile>,
    variant_pos: (usize, usize),
) -> bool {
    let mut tile_variant = tiles.clone();
    tile_variant.insert(variant_pos, Tile::Obstruction);

    let mut seen: HashSet<SeenTile> = HashSet::new();

    let mut finished = false;
    let max_steps = 20000;

    let mut steps = 0;
    while !finished && steps < max_steps {
        let result = step_part_2(&mut tile_variant, &mut seen, &size);
        finished = result.finished;

        if result.found_loop {
            return true;
        }

        steps += 1;
    }

    false
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
    // Note: 1396 too low

    #[test]
    fn test_part_two_small_loop() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(1));
    }


    #[test]
    fn test_part_two_small_loop2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(1));
    }
}
