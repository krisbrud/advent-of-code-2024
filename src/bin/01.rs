use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

advent_of_code::solution!(1);

lazy_static! {
    static ref regex_pattern: Regex = Regex::new(r"(\d+)   (\d+)").unwrap();
}

struct Line {
    left: i64,
    right: i64,
}

impl Line {
    fn parse(line: &str) -> Option<Line> {
        let captures = regex_pattern.captures(line)?;

        Some(Line {
            left: captures[1].parse().ok()?,
            right: captures[2].parse().ok()?,
        })
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let tuples = input.lines().map(Line::parse).collect::<Option<Vec<_>>>()?;

    let left: Vec<i64> = tuples.iter().map(|l| l.left).collect();
    let right: Vec<i64> = tuples.iter().map(|l| l.right).collect();

    let left_sorted: Vec<i64> = left.into_iter().sorted().collect();
    let right_sorted: Vec<i64> = right.into_iter().sorted().collect();

    let total_diff = left_sorted
        .into_iter()
        .zip(right_sorted.into_iter())
        .map(|(l, r)| (l-r).abs())
        .sum();

    Some(total_diff)
}

fn counts(numbers: Vec<i64>) -> HashMap<i64, i64> {
    let mut hashmap: HashMap<i64, i64> = HashMap::new();

    for number in &numbers {
        hashmap.entry(*number).and_modify(|entry| *entry += 1).or_insert(1);
    }

    hashmap
}

pub fn part_two(input: &str) -> Option<i64> {
    let tuples = input.lines().map(Line::parse).collect::<Option<Vec<_>>>()?;

    let right: Vec<i64> = tuples.iter().map(|l| l.right).collect();
    let right_counts = counts(right);

    let left: Vec<i64> = tuples.iter().map(|l| l.left).collect();

    let similarity: i64 = left
        .into_iter()
        .map(|num| {
            match right_counts.get(&num) {
                Some(&i) => num * i,
                None => 0
            }
        }).sum();
    
    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        let v: Vec<i64> = vec![3, 4, 3];
        let counts = counts(v);

        assert_eq!(Some(&2), counts.get(&3));
        assert_eq!(Some(&1), counts.get(&4));
        assert_eq!(None, counts.get(&1));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
