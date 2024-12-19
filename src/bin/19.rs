use memoize::memoize;

advent_of_code::solution!(19);

struct Problem {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Problem {
    fn new(s: &str) -> Option<Problem> {
        let patterns: Vec<String> = s
            .split("\n\n")
            .nth(0)?
            .split(", ")
            .map(|p| p.to_string())
            .collect();
        let designs = s
            .split("\n\n")
            .nth(1)?
            .split("\n")
            .map(|p| p.to_string())
            .collect();

        Some(Problem { patterns, designs })
    }
}

fn is_possible(design: &str, patterns: &Vec<String>) -> bool {
    if design == "" {
        // Base case, always possible
        return true;
    }

    let mut candidate_patterns = patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern));
    let rest_is_possible = candidate_patterns.any(|candidate_pattern| {
        let rest = &design[candidate_pattern.len()..];

        is_possible(rest, patterns)
    });
    rest_is_possible
}

#[memoize]
fn possible_combinations(design: String, patterns: Vec<String>) -> u64 {
    if design == "" {
        // Base case, always possible
        return 1;
    }

    let mut candidate_patterns = patterns
        .iter()
        .filter(|pattern| design.starts_with(*pattern)); // Implicitly filters out everything and makes the function return 0 if branch is not possible
    let possible_child_combinations = candidate_patterns.map(|candidate_pattern| {
        let rest = &design[candidate_pattern.len()..];

        let child_combinations = possible_combinations(rest.to_string(), patterns.clone());
        child_combinations
    });
    possible_child_combinations.sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let problem = Problem::new(input).expect("Input should parse");

    // For each design, it is possible if any of the subdesigns are possible when trying every feasible start pattern
    let num_possible: u32 = problem
        .designs
        .iter()
        .filter(|design| is_possible(design, &problem.patterns))
        .count()
        .try_into()
        .ok()?;

    Some(num_possible)
}

pub fn part_two(input: &str) -> Option<u64> {
    let problem = Problem::new(input).expect("Input should parse");

    // For each design, it is possible if any of the subdesigns are possible when trying every feasible start pattern
    let possible_combinations: u64 = problem
        .designs
        .iter()
        .map(|design| possible_combinations(design.to_string(), problem.patterns.clone()))
        .sum();

    Some(possible_combinations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
