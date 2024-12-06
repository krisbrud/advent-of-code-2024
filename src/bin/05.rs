advent_of_code::solution!(5);
use std::{collections::{HashMap, HashSet}, cmp::Ordering};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct PageOrderingRule {
    first: u32,
    after: u32,
}

#[derive(Debug, PartialEq)]
struct Update {
    pages: Vec<u32>,
}

#[derive(Debug, PartialEq)]
struct Problem {
    rules: Vec<PageOrderingRule>,
    updates: Vec<Update>,
}

lazy_static! {
    static ref regex_pattern: Regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
}

impl PageOrderingRule {
    fn parse(s: &str) -> Option<PageOrderingRule> {
        let caps = regex_pattern.captures(s)?;
        Some(PageOrderingRule {
            first: caps[1].parse().ok()?,
            after: caps[2].parse().ok()?,
        })
    }
}

impl Update {
    fn parse(s: &str) -> Option<Update> {
        let pages = s
            .split(",")
            .map(|s| {
                let out: Option<u32> = s.parse().ok();
                out
            })
            .collect::<Option<Vec<_>>>()?;

        Some(Update { pages })
    }

    fn is_valid(&self, rules: &HashSet<(u32, u32)>) -> bool {
        for i in 0..self.pages.len() - 1 {
            let first = self.pages[i];
            for j in i + 1..self.pages.len() {
                let second = self.pages[j];
                if !rules.contains(&(first, second)) {
                    return false;
                }
            }
        }

        return true;
    }

    fn middle(&self) -> u32 {
        let mid_index = self.pages.len() / 2;
        self.pages[mid_index]
    }
}

impl Problem {
    fn parse(s: &str) -> Option<Problem> {
        let first = s.split("\n\n").nth(0)?;
        let rules = first
            .lines()
            .map(PageOrderingRule::parse)
            .collect::<Option<Vec<_>>>()?;

        let second = s.split("\n\n").nth(1)?;
        let updates = second
            .lines()
            .map(Update::parse)
            .collect::<Option<Vec<_>>>()?;

        Some(Problem { rules, updates })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let problem = Problem::parse(input)?;

    let rules: HashSet<(u32, u32)> = problem
        .rules
        .iter()
        .map(|rule| (rule.first, rule.after))
        .collect();

    let valid_middle = problem
        .updates
        .iter()
        .filter(|update| update.is_valid(&rules))
        .map(|update| update.middle())
        .collect::<Vec<_>>();

    let sum: u32 = valid_middle.into_iter().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let problem = Problem::parse(input)?;

    let rules: HashSet<(u32, u32)> = problem
        .rules
        .iter()
        .map(|rule| (rule.first, rule.after))
        .collect();

    let invalid_updates = problem
        .updates
        .iter()
        .filter(|update| !update.is_valid(&rules))
        .collect::<Vec<_>>();

    let sorted_invalid_middle = invalid_updates
        .iter()
        .map(|update| {
            let mut pages = update.pages.clone();
            pages.sort_by(|a, b|  if !rules.contains(&(*a,*b)) {
                return Ordering::Less
            } else {
                return Ordering::Greater
            }
            );
            Update{ pages }
        })
        .map(|update| update.middle())
        .collect::<Vec<_>>();

    let sum: u32 = sorted_invalid_middle.into_iter().sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_page_ordering() {
        let result = PageOrderingRule::parse("75|13");
        assert_eq!(
            result,
            Some(PageOrderingRule {
                first: 75,
                after: 13
            })
        );
    }

    #[test]
    fn test_parse() {
        let result = Problem::parse("75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13");
        assert_eq!(
            result,
            Some(Problem {
                rules: vec![
                    PageOrderingRule {
                        first: 75,
                        after: 13
                    },
                    PageOrderingRule {
                        first: 53,
                        after: 13
                    }
                ],
                updates: vec![
                    Update {
                        pages: vec![75, 47, 61, 53, 29]
                    },
                    Update {
                        pages: vec![97, 61, 53, 29, 13]
                    }
                ]
            })
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
