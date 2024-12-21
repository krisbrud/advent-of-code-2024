use itertools::Itertools;

advent_of_code::solution!(15);

type Coordinate = (usize, usize);

struct Board {
    tiles: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn new(s: &str) -> Option<Board> {
        let tiles: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let rows = s.lines().collect_vec().len();
        let cols = s.lines().nth(0).map(|x| x.len())?;

        Some(Board { tiles, rows, cols })
    }

    fn get(&self, coordinate: &Coordinate) -> char {
        self.tiles[coordinate.0][coordinate.1]
    }

    fn set(&mut self, coordinate: &Coordinate, tile: char) {
        self.tiles[coordinate.0][coordinate.1] = tile
    }

    fn swap(&mut self, first: &Coordinate, second: &Coordinate) {
        let temp = self.get(first);
        self.set(first, self.get(second));
        self.set(second, temp)
    }

    fn as_string(&self) -> String {
        let mut out: String = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.push(self.get(&(row, col)))
            }
            out.push('\n');
        }

        out
    }
}

/** Preprocess the string.
If the tile is #, the new map contains ## instead.
If the tile is O, the new map contains [] instead.
If the tile is ., the new map contains .. instead.
If the tile is @, the new map contains @. instead.
 */
fn preprocess(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '#' => "##",
            'O' => "[]",
            '.' => "..",
            '@' => "@.",
            '\n' => "\n",
            _ => panic!("Unexpected char in preprocess!"),
        })
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn new(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::North),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            '>' => Some(Direction::East),
            _ => None,
        }
    }
}

// Coordinates in format (y, x)
fn coordinate_in_direction(coord: Coordinate, direction: Direction) -> Option<Coordinate> {
    match direction {
        Direction::North => Some((coord.0.checked_sub(1)?, coord.1)),
        Direction::South => Some((coord.0 + 1, coord.1)),
        Direction::East => Some((coord.0, coord.1 + 1)),
        Direction::West => Some((coord.0, coord.1.checked_sub(1)?)),
    }
}

/*
How to simulate a step:
- Find (or know) the position of the robot ('@')
- Parse the direction
- Iterate in that direction
    a: It has a wall ('#')
        -> Do nothing, done
    b: It is just air ('.')
        -> Move the agent there (swap agent and air chars)
    c: It has a box ('O')
        -> Check the next in that direction
            - It is still a box -> Keep going
            - It is a wall -> No move can be made, do nothing, done
            - It is air -> Want to move all the box.
                But this is a bit cumbersome, so it is better to
                1. Swap the box and the air
                2. Swap the air and the robot
 */

fn first_non_box_in_direction(
    board: &Board,
    coord: Coordinate,
    direction: Direction,
) -> (Coordinate, char) {
    let next_coord =
        coordinate_in_direction(coord, direction).expect("Shouldn't get out of bounds!");
    let next_tile = board.get(&next_coord);
    match next_tile {
        '.' | '#' => (next_coord, next_tile),
        'O' => first_non_box_in_direction(board, next_coord, direction),
        _ => panic!(
            "Finding first non-boulder - unexpected tile {} at coordinate {:?}",
            next_tile, next_coord
        ),
    }
}

// Returns new agent pos
fn step(board: &mut Board, agent_pos: Coordinate, direction: Direction) -> Coordinate {
    if let Some(front_coord) = coordinate_in_direction(agent_pos, direction) {
        match board.get(&front_coord) {
            '#' => return agent_pos, // Do nothing
            '.' => {
                // Move the agent to the empty air
                board.swap(&agent_pos, &front_coord);
                return front_coord;
            }
            'O' => {
                let (end_coord, end_tile) =
                    first_non_box_in_direction(board, front_coord, direction);
                if end_tile == '.' {
                    board.swap(&front_coord, &end_coord); // Swap box and air
                    board.swap(&agent_pos, &front_coord); // Swap agent and air
                    return front_coord;
                } else if end_tile == '#' {
                    return agent_pos; // Do nothing
                } else {
                    panic!("Unexpected end tile.")
                }
            }
            '@' => panic!("Unexpected agent!"),
            _ => panic!("Unknown tile!"),
        }
    }

    panic!("Couldn't find coord in direction!");
}

// (y, x)
fn find_agent_or_fail(board: &Board) -> (usize, usize) {
    for (i, row) in board.tiles.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == &'@' {
                return (i, j);
            }
        }
    }

    panic!("Failed to find agent")
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut board = Board::new(input.split("\n\n").nth(0)?)?;
    let directions = input.split("\n\n").nth(1)?;

    let mut agent_pos = find_agent_or_fail(&board);

    for direction_char in directions.lines().flat_map(|line| line.chars()) {
        // println!("Direction char: {}, agent_pos: {:?}", direction_char, agent_pos);
        // board.print();
        let direction = Direction::new(direction_char).expect("Should parse direction");
        agent_pos = step(&mut board, agent_pos, direction);
    }

    // Find sum of "GPS-coordinates"
    let gps_coordinate_sum: usize =
        board
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(row, row_tiles)| {
                row_tiles.into_iter().enumerate().map(move |(col, char)| {
                    if *char == 'O' {
                        (100 * row) + col
                    } else {
                        0
                    }
                })
            })
            .sum();

    Some(gps_coordinate_sum.try_into().expect("Should convert"))
}

/**
TODO:

Preprocess the board - DONE

A box is pushable if in a direction if the coordinates it will be pushed to
- Are air
- Are pushable boxes

Before we can execute a push, we need
- Left coordinate of all boxes (use this to calculate right coord)
- Direction

Executing a push:
- Copy all the left-hand-side box coordinates
- Also, find all the rhs coordinates
- Move the coordinates in direction and write
*/

fn get_rhs_coord(lhs_coord: Coordinate) -> Coordinate {
    coordinate_in_direction(lhs_coord, Direction::East).expect("Should find coordinate to the east")
}

fn get_lhs_coord(rhs_coord: Coordinate) -> Coordinate {
    coordinate_in_direction(rhs_coord, Direction::West).expect("Should find coordinate to the west")
}

fn pushable_boxes_in_direction(
    board: &Board,
    lhs_coord: Coordinate,
    direction: Direction,
) -> (Vec<Coordinate>, bool) {
    let rhs_coord = get_rhs_coord(lhs_coord);
    let next_coords = match direction {
        Direction::East => {
            vec![coordinate_in_direction(rhs_coord, direction).unwrap()]
        }
        Direction::West => {
            vec![coordinate_in_direction(lhs_coord, direction).unwrap()]
        }
        Direction::North | Direction::South => {
            vec![
                coordinate_in_direction(lhs_coord, direction).unwrap(),
                coordinate_in_direction(rhs_coord, direction).unwrap(),
            ]
        }
    };

    let mut pushable_boxes = vec![lhs_coord, rhs_coord];
    for next_coord in next_coords {
        let next_tile = board.get(&next_coord);
        match next_tile {
            '[' => {
                let (boxes, pushable) = pushable_boxes_in_direction(board, next_coord, direction);
                if !pushable {
                    return (vec![], false)
                }
                for b in boxes {
                    pushable_boxes.push(b);
                }
            },
            ']' => {
                let next_lhs_coord = get_lhs_coord(next_coord);
                let (boxes, pushable) = pushable_boxes_in_direction(board, next_lhs_coord, direction);
                if !pushable {
                    return (vec![], false)
                }
                for b in boxes {
                    pushable_boxes.push(b);
                }
            },
            '.' => {
                // Do nothing
            },
            '#' => {
                // Wall. No pushable boxes in direction
                return (vec![], false)
            },
            _ => panic!("Unexpected tile when finding pushable boxes")
        }
    }

    (pushable_boxes, true)
}

// Returns new agent pos
fn step_part_2(board: &mut Board, agent_pos: Coordinate, direction: Direction) -> Coordinate {
    if let Some(front_coord) = coordinate_in_direction(agent_pos, direction) {
        match board.get(&front_coord) {
            '#' => return agent_pos, // Do nothing
            '.' => {
                // Move the agent to the empty air
                board.swap(&agent_pos, &front_coord);
                return front_coord;
            }
            '[' => {
                let did_push = try_pushing_boxes(board, direction, front_coord);
                if did_push {
                    board.set(&front_coord, '@');
                    board.set(&agent_pos, '.');
                    return front_coord;
                } else {
                    return agent_pos;
                }
            }
            ']' => {
                // Same as above, but find the lhs coord first
                let next_lhs_coord = get_lhs_coord(front_coord);
                let did_push = try_pushing_boxes(board, direction, next_lhs_coord);
                if did_push {
                    board.set(&front_coord, '@');
                    board.set(&agent_pos, '.');
                    return front_coord;
                } else {
                    return agent_pos;
                }
            }
            '@' => panic!("Unexpected agent!"),
            _ => panic!("Unknown tile!"),
        }
    }

    panic!("Couldn't find coord in direction!");
}

// Returns whether boxes are pushed or not
fn try_pushing_boxes(board: &mut Board, direction: Direction, next_lhs_coord: (usize, usize)) -> bool {
    // Get pushable boxes in direction and pushability
    let (box_coords, pushable) = pushable_boxes_in_direction(board, next_lhs_coord, direction);

    // dbg!(box_coords.clone(), pushable);

    // If they are pushable
    if pushable {
        let current_coords_and_tiles = box_coords.iter().map(|coord| (coord, board.get(coord))).collect_vec();
        for coord in box_coords.clone() {
            // Set the coordinates to air
            board.set(&coord, '.');
        }

        // Find the new coordinates by pushing them all in the direction
        let pushed_coords_and_tiles = current_coords_and_tiles.into_iter().map(|((coord), tile)| {
            let pushed_coord = coordinate_in_direction(*coord, direction).expect("should be able to push box");
            (pushed_coord, tile)
        });
        // Write all the pushed coordinates back to the board
        for (pushed_coord, pushed_tile) in pushed_coords_and_tiles {
            board.set(&pushed_coord, pushed_tile)
        }
    }

    return pushable
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut board = Board::new(&preprocess(input.split("\n\n").nth(0)?))?;
    let directions = input.split("\n\n").nth(1)?;

    // Iteration:
    //
    let mut agent_pos = find_agent_or_fail(&board);

    // println!("initial state:\n{}", board.as_string());
    for direction_char in directions.lines().flat_map(|line| line.chars()) {
        // println!("Direction char: {}, agent_pos: {:?}", direction_char, agent_pos);
        // board.print();
        let direction = Direction::new(direction_char).expect("Should parse direction");
        agent_pos = step_part_2(&mut board, agent_pos, direction);
        // println!("move {}:\n{}", direction_char, board.as_string());
    }

    // // Find sum of "GPS-coordinates"
    let gps_coordinate_sum: usize =
        board
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(row, row_tiles)| {
                row_tiles.into_iter().enumerate().map(move |(col, char)| {
                    if *char == '[' {
                        (100 * row) + col
                    } else {
                        0
                    }
                })
            })
            .sum();

    Some(gps_coordinate_sum.try_into().expect("Should convert"))

    // None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_first_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_one_second_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(9021));
    }


    #[test]
    fn test_part_two_4() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
/*
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############

100 * 1 + 5 = 105
100 * 2 + 7 = 207
100 * 3 + 6 = 306
sum : 618
 */
        assert_eq!(result, Some(618));
    }

    #[test]
    fn test_preprocess() {
        let expected = advent_of_code::template::read_file_part("examples", DAY, 3);

        let actual = preprocess(
            advent_of_code::template::read_file_part("examples", DAY, 1)
                .split("\n\n")
                .nth(0)
                .unwrap(),
        );
        assert_eq!(expected, actual);
    }
}
