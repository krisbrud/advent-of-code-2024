use std::cmp::max;

advent_of_code::solution!(7);

#[derive(Debug, PartialEq)]
struct CalibrationEquation {
    test_value: u32,
    values: Vec<u32>,
}

impl CalibrationEquation {
    fn parse(line: &str) -> Option<CalibrationEquation> {
        let (left, right) = line.split_once(": ")?;
        let test_value: u32 = left.parse().ok()?;
        let values: Vec<u32> = right
            .split(" ")
            .map(|v| v.parse().ok())
            .collect::<Option<Vec<_>>>()?;

        Some(CalibrationEquation { test_value, values })
    }
}

fn calibration_result(lhs: Option<u32>, rest: Vec<u32>, test_value: u32) -> u32 {
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
            calibration_result(Some(lhs_ + next), next_rest.clone(), test_value),
            calibration_result(Some(lhs_ * next), next_rest, test_value),
        )
    } else {
        let next = rest.get(0).expect("rest must not be empty").to_owned();
        let next_rest = rest.get(1..).map_or(vec![], |x| x.to_vec());
        
        calibration_result(Some(next), next_rest, test_value)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let calibration_eqs = input
        .lines()
        .map(CalibrationEquation::parse)
        .collect::<Option<Vec<_>>>().expect("should parse");

    let total_calibration_result: u32 = calibration_eqs
        .iter()
        .map(|eq| calibration_result(None, eq.values.clone(), eq.test_value))
        .sum();

    Some(total_calibration_result)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
