use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(14);

type Position = (i64, i64);
type Velocity = (i64, i64);
type BoardSize = (i64, i64);

#[derive(Debug, PartialEq)]
struct Robot {
    pos: Position,
    vel: Velocity,
}

lazy_static! {
    static ref regex_pattern: Regex = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)").unwrap();
}


impl Robot {
    fn new(s: &str) -> Option<Robot> {
        let caps = regex_pattern.captures(s)?;

        Some(Robot {
            pos: (caps[2].parse().ok()?, caps[1].parse().ok()?),
            vel: (caps[4].parse().ok()?, caps[3].parse().ok()?),
        })
    }
}

fn simulate_single_dim(initial: i64, vel: i64, steps: i64, size: i64) -> i64 {
    let translation = vel * steps;
    let unwrapped_pos = initial + translation;
    let wrapped = unwrapped_pos % size;
    if wrapped >= 0 {
        return wrapped;
    } else {
        return size + wrapped;
    }
}

fn simulate(initial_pos: Position, vel: Velocity, steps: i64, rows: i64, cols: i64) -> Position {
    let row = simulate_single_dim(initial_pos.0, vel.0, steps, rows);
    let col = simulate_single_dim(initial_pos.1, vel.1, steps, cols);

    (row, col)
}

pub fn part_one(input: &str) -> Option<usize> {
    // let robots = input.lines().map(Robot::new).collect::<Option<Vec<_>>>()?;
    let robots = input.lines().map(|s| Robot::new(s)).collect::<Option<Vec<_>>>()?;

    let board_size: BoardSize = if robots.len() == 500 {
        (103, 101) // Input
    } else {
        (7, 11) // Example
    };

    let steps = 100;

    let final_positions: Vec<Position> = robots
        .iter()
        .map(|robot| simulate(robot.pos, robot.vel, steps, board_size.0, board_size.1)).collect();

    println!("Final positions: {:?}", final_positions);

    let middle_row = board_size.0 / 2;
    let middle_col = board_size.1 / 2;

    // Remove robots in the middle
    let filtered = final_positions.iter()
        .filter(|pos| pos.0 != middle_row)
        .filter(|pos| pos.1 != middle_col);

    println!("Filtered: {:?}", filtered);

    let robots_per_quadrant = filtered
        .into_grouping_map_by(|pos| ((pos.0 > middle_row), (pos.1 > middle_col)))
        .fold(0, |acc, _, _| {
            acc + 1
        }).values().copied().collect::<Vec<usize>>();

    println!("Robots per quadrant: {:?}", robots_per_quadrant);

    let safety_score = robots_per_quadrant.into_iter().reduce(|acc, e| acc * e)?;

    Some(safety_score)
    // None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let result = Robot::new("p=0,4 v=3,-3");
        assert_eq!(
            result,
            Some(Robot {
                pos: (4, 0),
                vel: (-3, 3)
            })
        );
    }

    #[test]
    fn test_simulate() {
        let actual = simulate((4, 2), (-3, 2), 5, 7, 11);
        let expected = (3,1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_all() {
        let input = advent_of_code::template::read_file("examples", DAY);
        input.lines().map(|s| Robot::new(s)).collect::<Option<Vec<_>>>().unwrap();
    }


    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
