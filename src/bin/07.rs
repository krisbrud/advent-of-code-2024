use std::cmp::max;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
struct CalibrationEquation {
    test_value: u64,
    values: Vec<u64>,
}

impl CalibrationEquation {
    fn parse(line: &str) -> Option<CalibrationEquation> {
        let (left, right) = line.split_once(": ")?;
        let test_value: u64 = left.parse().ok()?;
        let values: Vec<u64> = right
            .split(" ")
            .map(|v| v.parse().ok())
            .collect::<Option<Vec<_>>>()?;

        Some(CalibrationEquation { test_value, values })
    }
}

fn calibration_result_part_1(lhs: Option<u64>, rest: Vec<u64>, test_value: u64) -> u64 {
    if rest.len() == 0 {
        // Test if rest equals test_value
        if lhs.expect("lhs must not be empty") == test_value {
            return test_value;
        } else {
            return 0;
        }
    }

    if let Some(lhs_) = lhs {
        let next = rest.get(0).expect("rest must not be empty").to_owned();
        let next_rest = rest.get(1..).map_or(vec![], |x| x.to_vec());

        max(
            calibration_result_part_1(Some(lhs_ + next), next_rest.clone(), test_value),
            calibration_result_part_1(Some(lhs_ * next), next_rest, test_value),
        )
    } else {
        let next = rest.get(0).expect("rest must not be empty").to_owned();
        let next_rest = rest.get(1..).map_or(vec![], |x| x.to_vec());

        calibration_result_part_1(Some(next), next_rest, test_value)
    }
}

fn concatenate(left: u64, right: u64) -> u64 {
    let right_digits = right.checked_ilog10().unwrap_or(0) + 1;
    (left * 10u64.pow(right_digits)) + right
}

fn concatenate_first_two(lhs: u64, rest: Vec<u64>) -> Vec<u64> {
    match rest.len() {
        0 => vec![lhs],
        1 => vec![concatenate(lhs, rest[0])],
        _ => {
            let mut beginning = vec![concatenate(lhs, rest[0])];
            let mut rest = rest.get(1..).unwrap().to_vec();
            beginning.append(&mut rest);
            beginning
        }
    }
}

fn calibration_result_part_2(lhs: Option<u64>, rest: Vec<u64>, test_value: u64) -> u64 {
    if rest.len() == 0 {
        // Test if rest equals test_value
        if lhs.expect("lhs must not be empty") == test_value {
            return test_value;
        } else {
            return 0;
        }
    }

    if let Some(lhs_) = lhs {
        let next = rest.get(0).expect("rest must not be empty").to_owned();
        let next_rest = rest.get(1..).map_or(vec![], |x| x.to_vec());

        let concatenate_variant: u64 =
            calibration_result_part_2(Some(concatenate(lhs_, next)), next_rest.clone(), test_value);

        max(
            max(
                calibration_result_part_2(Some(lhs_ + next), next_rest.clone(), test_value),
                calibration_result_part_2(Some(lhs_ * next), next_rest, test_value),
            ),
            concatenate_variant,
        )
    } else {
        let next = rest.get(0).expect("rest must not be empty").to_owned();
        let next_rest = rest.get(1..).map_or(vec![], |x| x.to_vec());

        calibration_result_part_2(Some(next), next_rest, test_value)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let calibration_eqs = input
        .lines()
        .map(CalibrationEquation::parse)
        .collect::<Option<Vec<_>>>()
        .expect("should parse");

    let total_calibration_result: u64 = calibration_eqs
        .iter()
        .map(|eq| calibration_result_part_1(None, eq.values.clone(), eq.test_value))
        .sum();

    Some(total_calibration_result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let calibration_eqs = input
        .lines()
        .map(CalibrationEquation::parse)
        .collect::<Option<Vec<_>>>()
        .expect("should parse");

    let total_calibration_result: u64 = calibration_eqs
        .iter()
        .map(|eq| calibration_result_part_2(None, eq.values.clone(), eq.test_value))
        .sum();

    Some(total_calibration_result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let result = CalibrationEquation::parse("3267: 81 40 27");
        assert_eq!(
            result,
            Some(CalibrationEquation {
                test_value: 3267,
                values: vec![81, 40, 27],
            })
        );
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(156, concatenate(15, 6));
        assert_eq!(1560, concatenate(15, 60));
    }

    #[test]
    fn test_concatenate_first_two() {
        assert_eq!(vec![315, 6], concatenate_first_two(3, vec![15, 6]));
        assert_eq!(vec![3156], concatenate_first_two(3, vec![156]));
        assert_eq!(vec![3], concatenate_first_two(3, vec![]));
        assert_eq!(vec![315, 6, 4], concatenate_first_two(3, vec![15, 6, 4]));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_part_two_156() {
        assert_eq!(Some(156), part_two("156: 15 6"));
        // assert_eq!(Some(7290), part_two("7290: 6 8 6 15"));
        // assert_eq!(Some(192), part_two("192: 17 8 14"));
    }
}
