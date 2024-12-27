advent_of_code::solution!(24);

use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

// Avoid shenanigans from bitwise-xor
// fn xor(a: bool, b: bool) -> bool {
//     (a || b) && !(a && b)
// }

// fn and(a: bool, b: bool) -> bool {
//     a && b
// }

// fn or(a: bool, b: bool) -> bool {
//     a || b
// }

#[derive(Clone, Debug)]
enum Operator {
    XOR,
    AND,
    OR,
}

#[derive(Clone, Debug)]
struct Gate {
    in_a: String,
    in_b: String,
    out: String,
    operator: Operator,
}

fn digits(label: &str) -> Option<u32> {
    let caps = digit_pattern.captures(label)?;
    caps[1].parse().ok()
}

fn simulate(a: bool, b: bool, operator: Operator) -> bool {
    match operator {
        Operator::XOR => (a || b) && !(a && b),
        Operator::AND => a && b,
        Operator::OR => a || b,
    }
}

// x00 AND y00 -> z00

lazy_static! {
    static ref gate_pattern: Regex =
        Regex::new(r"([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z0-9]+)").unwrap();
    static ref digit_pattern: Regex = Regex::new(r"^z(\d\d)$").unwrap();
}
impl Gate {
    fn new(s: &str) -> Option<Gate> {
        let caps = gate_pattern.captures(s)?;

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

fn find_value(
    gate: Gate,
    all_gates: &HashMap<String, Gate>,
    given_values: &HashMap<String, bool>,
) -> bool {
    let a_value = *given_values.get(&gate.in_a).unwrap_or({
        // println!("{:?}", gate);
        let a_gate = all_gates.get(&gate.in_a).expect("Should find gate a");
        &find_value(a_gate.clone(), all_gates, given_values)
    });

    let b_value = *given_values.get(&gate.in_b).unwrap_or({
        let b_gate = all_gates.get(&gate.in_b).expect("Should find gate b");
        &find_value(b_gate.clone(), all_gates, given_values)
    });

    simulate(a_value, b_value, gate.operator)
}

pub fn part_one(input: &str) -> Option<u64> {
    // We need to simulate all the z wires
    // We can create some different maps
    // One map from output to the inputs
    // One map from wire to given literal values
    // One map from wire to deduced values

    // let given_values: Vec<(String, bool)> = input
    let given_values: HashMap<String, bool> = input
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
    let all_gates: HashMap<String, Gate> = gates
        .iter()
        .map(|gate| (gate.out.clone(), gate.clone()))
        .collect();

    // While there are unknown wire outputs

    let z_values: HashMap<String, bool> = gates
        .iter()
        .filter(|gate| gate.out.starts_with("z"))
        .cloned()
        .map(|gate| {
            (
                gate.out.clone(),
                find_value(gate, &all_gates, &given_values),
            )
        })
        .collect();

    let output: u64 = z_values.iter().filter(|(_, value)| **value).map(|(label, gate)| {
        let d = digits(label).expect("Should find digits");
        2u64.pow(d)
    }).sum();

    // let output = z_values.iter().filter(|(label, value)| {
    //     // let d = digits(**label).expect("should find digits");
    // })

    // How to simulate?
    // 1: Keep iterating every unknown gate
    // 2: Recursively find values from each z-gate

    Some(output)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, None);
    }
}
