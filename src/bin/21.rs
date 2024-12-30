advent_of_code::solution!(21);

/*
Numeric keypad
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+

(0,0): 7
(0,1): 8
(0,2): 9
(1,0): 4
(1,1): 5
(1,2): 6
(2,0): 1
(2,1): 2
(2,2): 3
(3,1): 0
(3,2): A
*/

/*
Directional keypad
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+

(0,1): ^
(0,2): A
(0,0): <
(0,1): v
(0,2): >
*/




pub fn part_one(input: &str) -> Option<u32> {
    // In summary, there are the following keypads:
    // - One directional keypad that you are using.
    // - Two directional keypads that robots are using.
    // - One numeric keypad (on a door) that a robot is using.
    // State space size (from numeric keypad):
    // (11, 5, 5) = 11 * 5 * 5 = 11 * 25 = 275

    // Should be feasible to use bfs for first part. But how do we do it if there are more?
    // If there were more keypads, the state space would have size 11 * 5^(numpads)
    // numpads == 10? => 11 * 5^10 = 11 * 9765625

    // A move is valid if:
    // Every robot is over a valid key

    // Note: Only one robot can move from a single human press to the keypad

    // Planning:
    // - Numeric: Should move horizontally first, then vertically
    // - Directional: Should move vertically then horizontally if on the top row,
    //   horizontal then vertical if in bottom row

    // Since we are starting with all robots hovering the A-buttons,
    // we can break down 029A to moving between
    // (A, A, A), (0, A, A), (2, A, A), (9, A, A), (A, A, A)
    // and pressing A on our directional pad after each

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
