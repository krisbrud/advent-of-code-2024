use itertools::Itertools;

advent_of_code::solution!(25);

#[derive(Clone, PartialEq, Eq, Debug)]
struct KeyComponent {
    columns: Vec<u32>,
    is_key: bool, // Keyhole if false
}

fn parse_columns(s: &str) -> Vec<u32> {
    let chars: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();
    dbg!(chars.clone());
    dbg!(chars.len());
    dbg!(chars.iter().nth(0).unwrap().len());
    let mut out: Vec<u32> = vec![];
    for c in 0..5 {
        for r in 0..6 {
            if chars[r + 1][c] == '.' {
                out.push(r.try_into().expect("Should convert"));
                break;
            }
        }
    }
    out
}

fn fits(key: &Vec<u32>, hole: &Vec<u32>) -> bool {
    for (key_col, hole_col) in key.iter().zip(hole.iter()) {
        if (key_col + hole_col) > 5 {
            return false;
        }
    }
    true
}

impl KeyComponent {
    fn new(s: &str) -> Option<KeyComponent> {
        let is_key = s.lines().nth(0)?.chars().all(|c| c == '.');

        let columns = if is_key {
            let reversed = s.lines().rev().collect_vec().join("\n");
            parse_columns(&reversed)
        } else {
            parse_columns(s)
        };

        Some(KeyComponent { columns, is_key })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let key_components = input
        .split("\n\n")
        .map(|line| KeyComponent::new(line))
        .collect::<Option<Vec<KeyComponent>>>()
        .expect("Should parse!");

    let keys = key_components
        .clone()
        .into_iter()
        .filter(|kc| kc.is_key)
        .collect_vec();
    let holes = key_components
        .clone()
        .into_iter()
        .filter(|kc| !kc.is_key)
        .collect_vec();

    let mut num_fits = 0;
    for key in &keys {
        for hole in &holes {
            if fits(&key.columns, &hole.columns) {
                num_fits += 1;
            }
        }
    }

    Some(num_fits)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lock() {
        let result = KeyComponent::new(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(
            result,
            Some(KeyComponent {
                columns: vec![0, 5, 3, 4, 3],
                is_key: false
            })
        );
    }

    #[test]
    fn test_parse_key() {
        let result = KeyComponent::new(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(
            result,
            Some(KeyComponent {
                columns: vec![5, 0, 2, 1, 3],
                is_key: true
            })
        );
    }

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(3));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
