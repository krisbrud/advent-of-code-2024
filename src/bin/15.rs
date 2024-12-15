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

    fn print(&self) {
        let mut out: String = String::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                out.push(self.get(&(row, col)))
            }
            out.push('\n');
        }

        println!("{}", out);
    }
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
    c: It has a boulder ('O')
        -> Check the next in that direction
            - It is still a boulder -> Keep going
            - It is a wall -> No move can be made, do nothing, done
            - It is air -> Want to move all the boulders.
                But this is a bit cumbersome, so it is better to
                1. Swap the boulder and the air
                2. Swap the air and the robot
 */

fn first_non_boulder_in_direction(
    board: &Board,
    coord: Coordinate,
    direction: Direction,
) -> (Coordinate, char) {
    let next_coord =
        coordinate_in_direction(coord, direction).expect("Shouldn't get out of bounds!");
    let next_tile = board.get(&next_coord);
    match next_tile {
        '.' | '#' => (next_coord, next_tile),
        'O' => first_non_boulder_in_direction(board, next_coord, direction),
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
                    first_non_boulder_in_direction(board, front_coord, direction);
                if end_tile == '.' {
                    board.swap(&front_coord, &end_coord); // Swap boulder and air
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}
