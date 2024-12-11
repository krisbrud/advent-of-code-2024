use memoize::memoize;

advent_of_code::solution!(11);

fn number_of_digits(stone: u64) -> usize {
    (stone.checked_ilog10().unwrap_or(0) + 1) as usize
}

fn split_stone(stone: u64) -> Vec<u64> {
    let x = 10u64.pow((number_of_digits(stone) / 2) as u32);

    // 1234 -> 12, 34
    let first = stone / x;
    let second = stone % x;

    return vec![first, second];
}

// Every time you blink, the stones each simultaneously change according to the first applicable rule in this list:
// - If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
// - If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
//   The left half of the digits are engraved on the new left stone,
//   and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
// - If none of the other rules apply, the stone is replaced by a new stone;
//   the old stone's number multiplied by 2024 is engraved on the new stone.
fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    if (number_of_digits(stone) % 2) == 0 {
        return split_stone(stone);
    }

    return vec![stone * 2024];
}

#[memoize]
fn stones_after_blinking_n_times(stone: u64, times: u32) -> u64 {
    if times == 0 {
        return 1;
    }

    blink(stone)
        .into_iter()
        .map(|s| stones_after_blinking_n_times(s, times - 1))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let initial_stones = input
        .split(" ")
        .map(|s| s.parse::<u64>().ok())
        .collect::<Option<Vec<_>>>()?;

    let final_stone_count: u64 = initial_stones
        .iter()
        .map(|stone| stones_after_blinking_n_times(*stone, 25))
        .sum();

    Some(final_stone_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial_stones = input
        .split(" ")
        .map(|s| s.parse::<u64>().ok())
        .collect::<Option<Vec<_>>>()?;

    let final_stone_count: u64 = initial_stones
        .iter()
        .map(|stone| stones_after_blinking_n_times(*stone, 75))
        .sum();

    Some(final_stone_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split() {
        assert_eq!(vec![1, 2], split_stone(12));
        assert_eq!(vec![12, 34], split_stone(1234));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
