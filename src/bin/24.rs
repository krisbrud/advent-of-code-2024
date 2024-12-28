advent_of_code::solution!(24);

use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

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
    let a = if let Some(a_value) = given_values.get(&gate.in_a) {
        *a_value
    } else {
        let a_gate = all_gates.get(&gate.in_a).expect("Should find gate a");
        find_value(a_gate.clone(), all_gates, given_values)
    };

    let b = if let Some(b_value) = given_values.get(&gate.in_b) {
        *b_value
    } else {
        let b_gate = all_gates.get(&gate.in_b).expect("Should find gate b");
        find_value(b_gate.clone(), all_gates, given_values)
    };

    simulate(a, b, gate.operator)
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

    // dbg!(gates.clone());

    let all_gates: HashMap<String, Gate> = gates
        .iter()
        .map(|gate| (gate.out.clone(), gate.clone()))
        .collect();

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

    Some(output)
}

fn apply_overrides(gate: Gate) -> Gate {
    match &gate.out {

    }

}

pub fn part_two(input: &str) -> Option<u32> {
    // Other interesting rows:
    // y06 XOR x06 -> gbp
    // y11 XOR x11 -> sqv
    // y16 XOR x16 -> vgv

    // frp XOR sqv -> tqm
    // scp XOR gbp -> vwr
    // vgv XOR hpt -> kfs

    // x05 XOR y05 -> mpk

    // TODO
    // Make a function to swap the known swapped wires
    // Make a function to create the bit array from a u64
    // Make a function to create the u64 from a bit array

    // The following _must_ be wrong
    // x06 AND y06 -> z06
    // sqv AND frp -> z11
    // bmp OR vjc -> z16

    // Need following values:
    // y05 AND x05 -> qjv
    // x10 AND y10 -> jpc
    // y15 AND x15 -> ksg

    // The following _must_ be wrong
    // x06 AND y06 -> z06
    // y05 AND x05 -> qjv

    // x10 AND y10 -> jpc
    // sqv AND frp -> z11

    // y15 AND x15 -> ksg
    // bmp OR vjc -> z16

    // This one should actually be there
    // gwr OR mmj -> z45
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
