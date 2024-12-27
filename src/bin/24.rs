advent_of_code::solution!(24);

use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

// Avoid shenanigans from bitwise-xor
fn xor(a: bool, b: bool) -> bool {
    (a || b) && !(a && b)
}

fn and(a: bool, b: bool) -> bool {
    a && b
}

fn or(a: bool, b: bool) -> bool {
    a || b
}

#[derive(Clone)]
enum Operator {
    XOR,
    AND,
    OR,
}

#[derive(Clone)]
struct Gate {
    in_a: String,
    in_b: String,
    out: String,
    operator: Operator,
}

// x00 AND y00 -> z00

lazy_static! {
    static ref regex_pattern: Regex =
        Regex::new(r"([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z0-9]+)").unwrap();
}
impl Gate {
    fn new(s: &str) -> Option<Gate> {
        let caps = regex_pattern.captures(s)?;

        let operator = match &caps[2] {
            "AND" => Operator::AND,
            "XOR" => Operator::XOR,
            "OR" => Operator::OR,
            _ => {
                return None;
            }
        };

        Some(Gate {
            in_a: caps[1].to_string(),
            in_b: caps[3].to_string(),
            out: caps[4].to_string(),
            operator,
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    // We need to simulate all the z wires
    // We can create some different maps
    // One map from output to the inputs
    // One map from wire to given literal values
    // One map from wire to deduced values

    let given_values: Vec<(String, bool)> = input
        .split("\n\n")
        .nth(0)?
        .lines()
        .map(|line| {
            let wire = line.split(": ").collect_vec()[0];
            let value = line.split(": ").collect_vec()[1] == "1";
            (wire.to_string(), value)
        })
        .collect();

    let mut known_values: HashMap<String, bool> = given_values.clone().into_iter().collect();

    // Make a map of wires and their known values
    let gates = input
        .split("\n\n")
        .nth(1)?
        .lines()
        .map(|line| Gate::new(line))
        .collect::<Option<Vec<Gate>>>()?;

    // let mut unknown_gates = gates.clone();

    // While there are unknown wire outputs



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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
