use std::{collections::HashMap, iter::zip};

use itertools::Itertools;

advent_of_code::solution!(22);

fn mix(given: u64, secret: u64) -> u64 {
    given ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn next_secret(secret: u64) -> u64 {
    // Calculate the result of multiplying the secret number by 64.
    // Then, mix this result into the secret number. Finally, prune the secret number.
    let res1 = secret * 64;
    let res2 = mix(res1, secret);
    let res3 = prune(res2);

    // Calculate the result of dividing the secret number by 32.
    // Round the result down to the nearest integer.
    // Then, mix this result into the secret number. Finally, prune the secret number.
    let res4 = res3 / 32;
    let res5 = prune(mix(res4, res3));

    // Calculate the result of multiplying the secret number by 2048.
    // Then, mix this result into the secret number. Finally, prune the secret number.
    let res6 = res5 * 2048;
    prune(mix(res6, res5))
}

fn price(secret: u64) -> u64 {
    secret % 10
}

pub fn part_one(input: &str) -> Option<u64> {
    let steps = 2000;
    let initial_secrets: Vec<u64> = input
        .lines()
        .map(|x| x.parse().ok())
        .collect::<Option<Vec<_>>>()?;
    let final_secrets = initial_secrets.iter().map(|initial_secret| {
        let mut secret = *initial_secret;
        for _ in 0..steps {
            secret = next_secret(secret);
        }
        secret
    });
    let secret_sum: u64 = final_secrets.sum();
    Some(secret_sum)
}

type Bid = (i64, i64, i64, i64);

pub fn part_two(input: &str) -> Option<i64> {
    // TODO
    let steps = 2000;
    let initial_secrets: Vec<u64> = input
        .lines()
        .map(|x| x.parse().ok())
        .collect::<Option<Vec<_>>>()?;
    // Simulate the secrets of each monkey for 2000 steps
    let all_secrets: Vec<Vec<u64>> = initial_secrets
        .iter()
        .map(|initial_secret| {
            let mut secrets = vec![];
            let mut secret = *initial_secret;
            secrets.push(secret);
            for _ in 0..steps {
                secret = next_secret(secret);
                secrets.push(secret);
            }
            secrets
        })
        .collect();

    // Calculate the prices from the secrets
    let all_prices: Vec<Vec<i64>> = all_secrets
        .iter()
        .map(|monkey_secrets| {
            monkey_secrets
                .iter()
                .map(|secret| {
                    let unsigned_price = price(*secret);
                    let signed_price: i64 =
                        unsigned_price.try_into().expect("Should convert price");
                    signed_price
                })
                .collect()
        })
        .collect();
    // Calculate the diffs from the prices
    let price_changes: Vec<Vec<i64>> = all_prices
        .iter()
        .map(|monkey_prices| {
            monkey_prices
                .iter()
                .tuple_windows::<(_, _)>()
                .map(|(first, second)| second - first)
                .collect()
        })
        .collect();

    // Calculate all the bids and prices (i64, i64, i64, i64), i64 for each monkey
    let bids: Vec<Vec<Bid>> = price_changes
        .iter()
        .map(|monkey_price_changes| {
            monkey_price_changes
                .iter()
                .cloned()
                .tuple_windows::<(_, _, _, _)>()
                .collect()
        })
        .collect();

    let bid_prices: Vec<Vec<i64>> = all_prices
        .iter()
        .map(|monkey_prices| monkey_prices.as_slice()[4..].to_vec())
        .collect();
    // -> How do we do this?
    // For each monkey, take the first price for each unique bid -> HashMap
    let mut first_bids: Vec<HashMap<Bid, i64>> = vec![];
    for (monkey_bids, monkey_prices) in zip(bids, bid_prices) {
        let mut monkey_first_bids: HashMap<Bid, i64> = HashMap::new();
        for (bid, price) in zip(monkey_bids, monkey_prices) {
            if !monkey_first_bids.contains_key(&bid) {
                monkey_first_bids.insert(bid, price);
            }
        }
        first_bids.push(monkey_first_bids);
    }

    let all_bids_and_prices: Vec<(Bid, i64)> = first_bids
        .clone()
        .into_iter()
        .flat_map(|monkey_first_bids| monkey_first_bids)
        .collect();

    // For each bid, sum all the first prices
    let bananas_by_bid = all_bids_and_prices
        .clone()
        .into_iter()
        .into_grouping_map()
        .reduce(|price_acc, _, price| price_acc + price);

    let max_bananas = *bananas_by_bid.values().max()?;

    Some(max_bananas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(23));
    }
}
