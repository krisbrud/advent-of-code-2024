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

fn find_value_part_2(
    gate: Gate,
    all_gates: &HashMap<String, Gate>,
    given_values: &HashMap<String, bool>,
) -> bool {
    let a = if let Some(a_value) = given_values.get(&gate.in_a) {
        *a_value
    } else {
        if let Some(a_gate) = all_gates.get(&gate.in_a) {
            find_value_part_2(a_gate.clone(), all_gates, given_values)
        } else {
            false
        }
    };

    let b = if let Some(b_value) = given_values.get(&gate.in_b) {
        *b_value
    } else {
        if let Some(b_gate) = all_gates.get(&gate.in_b) {
            find_value_part_2(b_gate.clone(), all_gates, given_values)
        } else {
            false
        }
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

    let output: u64 = z_values
        .iter()
        .filter(|(_, value)| **value)
        .map(|(label, gate)| {
            let d = digits(label).expect("Should find digits");
            2u64.pow(d)
        })
        .sum();

    Some(output)
}

fn with_output(gate: Gate, out: &str) -> Gate {
    let mut new_gate = gate.clone();
    new_gate.out = out.to_string();
    new_gate
}

fn apply_overrides(gate: Gate) -> Gate {
    match gate.out.as_str() {
        "z06" => with_output(gate, "vwr"),
        "vwr" => with_output(gate, "z06"),

        "z11" => with_output(gate, "tqm"),
        "tqm" => with_output(gate, "z11"),

        "z16" => with_output(gate, "kfs"),
        "kfs" => with_output(gate, "z16"),

        "gfv" => with_output(gate, "hcm"),
        "hcm" => with_output(gate, "gfv"),

        _ => gate,
    }
}

fn u64_to_bits(num: u64) -> Vec<bool> {
    let mut out = vec![];
    let mut x = num;
    while x != 0 {
        out.push(x % 2 == 1);
        x = x / 2;
    }
    out
}

fn bits_to_pairs(bits: Vec<bool>, prefix: &str) -> Vec<(String, bool)> {
    bits.iter()
        .enumerate()
        .map(|(i, value)| {
            let out = format!("{}{:02}", prefix, i);
            (out, *value)
        })
        .collect()
}

fn simulate_part_2(input: &str, x_value: u64, y_value: u64) -> Option<u64> {
    // let x_value = 2u64.pow(45) - 1;
    let x_bits = u64_to_bits(x_value);
    let x_given_values = bits_to_pairs(x_bits.clone(), "x");

    // let y_value =  1;
    let y_bits = u64_to_bits(y_value);
    let y_given_values = bits_to_pairs(y_bits.clone(), "y");

    // let mut known_values: HashMap<String, bool> = x_given_values.clone().into_iter().chain(y_given_values.clone().into_iter()).collect();
    let given_values: HashMap<String, bool> = x_given_values
        .clone()
        .into_iter()
        .chain(y_given_values.clone().into_iter())
        .collect();

    // Make a map of wires and their known values
    let raw_gates = input
        .split("\n\n")
        .nth(1)?
        .lines()
        .map(|line| Gate::new(line))
        .collect::<Option<Vec<Gate>>>()?;

    let gates = raw_gates
        .iter()
        .map(|gate| apply_overrides(gate.clone()))
        .collect_vec();

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
                find_value_part_2(gate, &all_gates, &given_values),
            )
        })
        .collect();

    let output: u64 = z_values
        .iter()
        .filter(|(_, value)| **value)
        .map(|(label, gate)| {
            let d = digits(label).expect("Should find digits");
            2u64.pow(d)
        })
        .sum();

    println!("x : {:#048b}", x_value);
    println!("y : {:#048b}", y_value);
    println!("z': {:#048b}", output);
    println!("z : {:#048b}", x_value + y_value);
    println!("");

    // let actual_z_bits = u64_to_bits(output);
    // let expected_z_bits = u64_to_bits(x_value + y_value);

    // println!("x_bits: {:?}", x_bits);
    // println!("y_bits: {:?}", y_bits);
    // println!("z act:  {:?}", actual_z_bits);
    // println!("z gt:   {:?}", expected_z_bits);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    // let x_value = 2u64.pow(45) - 1;
    // let y_value =  1;
    let pairs: Vec<(u64, u64)> = vec![
        (2u64.pow(45) - 1, 1),
        (2u64.pow(40) - 1, 1),
        (2u64.pow(6) - 1, 1),
        (2u64.pow(11) - 1, 1),
        (2u64.pow(16) - 1, 1),
    ];

    for (x_value, y_value) in pairs {
        // simulate_part_2(raw_gates, x_value, y_value);
        simulate_part_2(input, x_value, y_value);
    }

    None

    // Last gate
    // y37 XOR x37 -> fcw - this value - seems good?
    // x36 AND y36 -> hcm - carry
    // thv AND hcm -> wpj - thv should have been the previous AND value
    // gfv OR wpj -> jgk
    // fcw XOR jgk -> z37

    // gfv and hcm are swapped!

    // jgk AND fcw -> qbd
    // qbg OR qbd -> vgg

    // y36 XOR x36 -> gfv - value 36 before prev carry
    // y35 AND x35 -> jgm
    // thv XOR hcm -> z36
    // hmg OR jgm -> thv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, None);
    }
}
