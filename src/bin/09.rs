use core::num;
use std::mem::swap;

use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Clone, Copy)]
struct DiskMapEntry {
    blocks: u8,
    skip: Option<u8>,
}

impl DiskMapEntry {
    fn parse(s: Vec<char>) -> Option<DiskMapEntry> {
        match s.len() {
            2 => Some(DiskMapEntry {
                blocks: s[0].to_string().parse().ok()?,
                skip: Some(s[1].to_string().parse().ok()?),
            }),
            1 => Some(DiskMapEntry {
                blocks: s[0].to_string().parse().ok()?,
                skip: None,
            }),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct DiskMap {
    entries: Vec<DiskMapEntry>,
}

impl DiskMap {
    fn parse(s: &str) -> Option<DiskMap> {
        let entries = s
            .chars()
            .chunks(2)
            .into_iter()
            .map(|spec| {
                let chars = spec.collect::<Vec<char>>();
                DiskMapEntry::parse(chars)
            })
            .collect::<Option<Vec<_>>>()?;
        Some(DiskMap { entries })
    }

    fn spread(&self) -> Vec<Option<u32>> {
        let mut out: Vec<Option<u32>> = vec![];
        for (id, entry) in self.entries.iter().enumerate() {
            for _ in 0..entry.blocks {
                out.push(Some(id as u32))
            }

            // Last entry will not have skip
            if let Some(skip) = entry.skip {
                for _ in 0..skip {
                    out.push(None);
                }
            }
        }
        out
    }
}

fn checksum(numbers: Vec<u32>) -> u64 {
    let out = numbers
        .iter()
        .enumerate()
        .map(|(i, x)| (u64::try_from(i).unwrap()) * (u64::try_from(*x).unwrap()))
        .sum();
    out
}

pub fn part_one(input: &str) -> Option<u64> {
    let diskmap = DiskMap::parse(input)?;
    let mut spreaded = diskmap.spread();

    // Dual pointer approach
    let mut l = 0;
    let mut r = spreaded.len() - 1;

    while l < r {
        while spreaded[l].is_some() && l < r {
            l += 1;
        }

        while spreaded[r].is_none() && l < r {
            r -= 1
        }
        if l < r {
            spreaded.swap(l, r);
        }
    }

    let (first_none_idx, _) = spreaded.iter().find_position(|x| x.is_none())?;
    let numbers = spreaded
        .iter()
        .take(first_none_idx)
        .copied()
        .collect::<Option<Vec<u32>>>()?;

    Some(checksum(numbers))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_spread() {
        let diskmap = DiskMap::parse("12345").unwrap();
        assert_eq!(
            vec![
                Some(0),
                None,
                None,
                Some(1),
                Some(1),
                Some(1),
                None,
                None,
                None,
                None,
                Some(2),
                Some(2),
                Some(2),
                Some(2),
                Some(2)
            ],
            diskmap.spread()
        );
    }

    #[test]
    fn test_checksum() {
        let numbers = vec![0,0,9,9,8,1,1,1,8,8,8,2,7,7,7,3,3,3,6,4,4,6,5,5,5,5,6,6];
        assert_eq!(1928, checksum(numbers))
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
