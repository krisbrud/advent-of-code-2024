use std::collections::VecDeque;

use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// struct DiskMapEntry {
//     blocks: u64,
//     skip: u64,
// }
struct DiskMapEntry {
    blocks: usize,
    skip: usize,
}

impl DiskMapEntry {
    fn parse(s: Vec<char>) -> Option<DiskMapEntry> {
        match s.len() {
            2 => Some(DiskMapEntry {
                blocks: s[0].to_string().parse().ok()?,
                skip: s[1].to_string().parse().ok()?,
            }),
            1 => Some(DiskMapEntry {
                blocks: s[0].to_string().parse().ok()?,
                skip: 0,
            }),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct IdentifiedEntry {
    id: usize,
    entry: DiskMapEntry,
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
            for _ in 0..entry.skip {
                out.push(None);
            }
        }
        out
    }
}

// fn spread_part_2(id_entries: &VecDeque<IdentifiedEntry>) -> Vec<Option<u64>> {
fn spread_part_2(id_entries: &Vec<IdentifiedEntry>) -> Vec<Option<u64>> {
    let mut out: Vec<Option<u64>> = vec![];
    for id_entry in id_entries.iter() {
        for _ in 0..id_entry.entry.blocks {
            out.push(Some(id_entry.id.try_into().expect("Should convert u32 to u64")));
        }

        // Last entry will not have skip
        for _ in 0..id_entry.entry.skip {
            out.push(None);
        }
    }
    out
}

fn spread_checksum_part_2(id_entries: &Vec<IdentifiedEntry>) -> usize {
    let mut checksum = 0;
    let mut pos: usize = 0;
    for id_entry in id_entries.iter() {
        for i in 0..id_entry.entry.blocks {
            let contr = (pos + i) * id_entry.id;
            checksum += contr;
        }
        pos += id_entry.entry.blocks + id_entry.entry.skip;
    }
    checksum
}

fn checksum(numbers: Vec<u32>) -> u64 {
    let mut out: u64 = 0;
    for (pos_usize, idu32) in numbers.iter().enumerate() {
        let id: u64 = u64::try_from(*idu32).unwrap();
        let pos: u64 = pos_usize.try_into().unwrap();
        let contribution = id * pos;
        out += contribution
    }
    // let out = numbers
    //     .iter()
    //     .enumerate()
    //     .map(|(i, x)| (u64::try_from(i).unwrap()) * (u64::try_from(*x).unwrap()))
    //     .sum();
    out
}

fn checksum_part_2(numbers: Vec<Option<u64>>) -> u64 {
    let out = numbers
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if let Some(id) = x {
                let pos: u64 = i.try_into().unwrap();
                pos * id
            } else { 0 }
        })
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

fn as_string(vec: &Vec<Option<u64>>) -> String {
    let mut out: Vec<char> = vec![];
    for maybe_id in vec {
        if let Some(id_entry) = maybe_id {
            let s = id_entry.to_string();
            for c in s.chars() {
                out.push(c);
            }
        } else {
            out.push('.');
        }
    }
    out.into_iter().collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let diskmap = DiskMap::parse(input)?;

    let entries = diskmap
        .entries
        .into_iter()
        .collect::<VecDeque<DiskMapEntry>>();

    let mut identified_entries: Vec<IdentifiedEntry> = entries
        .iter()
        .enumerate()
        .map(|(id, entry)| IdentifiedEntry { id, entry: entry.clone() })
        .collect();

    let num_entries = entries.len();

    for id_to_move in (0..num_entries).rev() {
        let (mover_idx, mover_identified_entry) = identified_entries
            .clone()
            .into_iter()
            .find_position(|entry| entry.id == id_to_move)
            .expect("Should find entry to move");

        if let Some((target_idx, target_identified_entry)) =
            identified_entries.clone().into_iter().take(mover_idx).find_position(|target_candidate| {
                target_candidate.entry.skip >= mover_identified_entry.entry.blocks
            })
        {
            let mut new_mover = mover_identified_entry.clone();
            let before_mover_idx = mover_idx - 1;
            identified_entries[before_mover_idx].entry.skip += new_mover.entry.blocks + new_mover.entry.skip;

            let rest = identified_entries[target_idx].entry.skip.checked_sub(new_mover.entry.blocks).expect("rest shouldn't underflow");

            identified_entries[target_idx].entry.skip = 0;
            new_mover.entry.skip = rest;

            identified_entries.remove(mover_idx); // Note: since target is always to the left, we need to remove this one first

            identified_entries.insert(target_idx + 1, new_mover);
        }
    }

    Some(spread_checksum_part_2(&identified_entries).try_into().expect("Should convert"))
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
        let numbers = vec![
            0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
        ];
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
        // assert_eq!(result, Some(2858));
        assert_eq!(result, None);
    }
}
