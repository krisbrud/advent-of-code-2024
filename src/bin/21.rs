advent_of_code::solution!(21);

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use lazy_static::lazy_static;

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

type Coord = (usize, usize);

fn coord_in_direction(coord: Coord, direction: Direction) -> Option<Coord> {
    match direction {
        Direction::Up => Some((coord.0.checked_sub(1)?, coord.1)),
        Direction::Down => Some((coord.0 + 1, coord.1)),
        Direction::Right => Some((coord.0, coord.1 + 1)),
        Direction::Left => Some((coord.0, coord.1.checked_sub(1)?)),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn dirpad_to_direction(dirpad: &DirPad) -> Option<Direction> {
    match dirpad {
        DirPad::Up => Some(Direction::Up),
        DirPad::Left => Some(Direction::Left),
        DirPad::Down => Some(Direction::Down),
        DirPad::Right => Some(Direction::Right),
        DirPad::A => None,
    }
}

lazy_static! {
    static ref numpad_from_coord: HashMap<Coord, NumPad> = vec![
        ((0, 0), NumPad::Seven),
        ((0, 1), NumPad::Eight),
        ((0, 2), NumPad::Nine),
        ((1, 0), NumPad::Four),
        ((1, 1), NumPad::Five),
        ((1, 2), NumPad::Six),
        ((2, 0), NumPad::One),
        ((2, 1), NumPad::Two),
        ((2, 2), NumPad::Three),
        ((3, 1), NumPad::Zero),
        ((3, 2), NumPad::A)
    ]
    .into_iter()
    .collect();
    static ref coord_from_numpad: HashMap<NumPad, Coord> = numpad_from_coord
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect();
    static ref dirpad_from_coord: HashMap<Coord, DirPad> = vec![
        ((0, 1), DirPad::Up),
        ((0, 2), DirPad::A),
        ((1, 0), DirPad::Left),
        ((1, 1), DirPad::Down),
        ((1, 2), DirPad::Right),
    ]
    .into_iter()
    .collect();
    static ref coord_from_dirpad: HashMap<DirPad, Coord> = dirpad_from_coord
        .iter()
        .map(|(k, v)| (v.clone(), k.clone()))
        .collect();
    static ref dirpad_values: Vec<DirPad> = vec![
        DirPad::Left,
        DirPad::Down,
        DirPad::Right,
        DirPad::Up,
        DirPad::A
    ];
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
enum NumPad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
enum DirPad {
    Up,
    A,
    Left,
    Down,
    Right,
}

struct NumpadMoveResult {
    next_numpad: NumPad,
    pressed_numpad_button: bool,
}

// Returns None if the command is invalid (i.e. the numpad arm is out of bounds)
fn do_numpad_arm_command(numpad: NumPad, command: DirPad) -> Option<NumpadMoveResult> {
    let current_numpad_coord = coord_from_numpad
        .get(&numpad)
        .expect("Should have all numpads");
    match command {
        DirPad::Up | DirPad::Left | DirPad::Down | DirPad::Right => {
            let direction = dirpad_to_direction(&command).expect("Should find direction");
            let next_coord = coord_in_direction(*current_numpad_coord, direction)?;
            let next_numpad = *numpad_from_coord.get(&next_coord)?; // Returns None if out of bounds

            Some(NumpadMoveResult {
                next_numpad,
                pressed_numpad_button: false,
            })
        }
        DirPad::A => Some(NumpadMoveResult {
            next_numpad: numpad,
            pressed_numpad_button: true,
        }),
    }
}

struct DirPadMoveResult {
    next_dirpad: DirPad,
    pressed_dirpad_button: Option<DirPad>,
}

// Returns None if the command is invalid (i.e. the dirpad arm is out of bounds)
fn do_dirpad_arm_command(dirpad: DirPad, command: DirPad) -> Option<DirPadMoveResult> {
    let current_dirpad_coord = coord_from_dirpad
        .get(&dirpad)
        .expect("Should have all numpads");
    match command {
        DirPad::Up | DirPad::Left | DirPad::Down | DirPad::Right => {
            let direction = dirpad_to_direction(&command).expect("Should find direction");
            let next_coord = coord_in_direction(*current_dirpad_coord, direction)?;
            let next_dirpad = *dirpad_from_coord.get(&next_coord)?; // Returns None if out of bounds

            Some(DirPadMoveResult {
                next_dirpad,
                pressed_dirpad_button: None,
            })
        }
        DirPad::A => Some(DirPadMoveResult {
            next_dirpad: dirpad,
            pressed_dirpad_button: Some(dirpad),
        }),
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
struct State {
    numpad: NumPad,

    // First element is the one pressed by humans, the last is the one pressing the numpad
    directional_pads: Vec<DirPad>,

    pressed_numpad_button: bool,
}

fn do_command(state: &State, human_command: &DirPad) -> Option<State> {
    // println!("Doing command {:?}", human_command);
    let mut maybe_command = Some(human_command.clone());
    let mut next_directional_pads: Vec<DirPad> = vec![];

    for dirpad in state.directional_pads.iter() {
        if let Some(command) = maybe_command {
            let result = do_dirpad_arm_command(*dirpad, command)?; // Returns None if this goes out of bounds
            maybe_command = result.pressed_dirpad_button;
            next_directional_pads.push(result.next_dirpad);
        } else {
            next_directional_pads.push(*dirpad);
        }
    }

    let (next_numpad, pressed_numpad_button) = if let Some(command) = maybe_command {
        // Move numpad
        let result = do_numpad_arm_command(state.numpad, command)?; // Returns None if this goes out of bounds
        (result.next_numpad, result.pressed_numpad_button)
    } else {
        // No move
        (state.numpad, false)
    };

    // println!("At end of doing command {:?}", human_command);

    Some(State {
        numpad: next_numpad,
        directional_pads: next_directional_pads,
        pressed_numpad_button,
    })
}

fn parse_numpad(c: &char) -> NumPad {
    match c {
        '0' => NumPad::Zero,
        '1' => NumPad::One,
        '2' => NumPad::Two,
        '3' => NumPad::Three,
        '4' => NumPad::Four,
        '5' => NumPad::Five,
        '6' => NumPad::Six,
        '7' => NumPad::Seven,
        '8' => NumPad::Eight,
        '9' => NumPad::Nine,
        'A' => NumPad::A,
        _ => {
            panic!("{}", format!("Illegal char {}!", c))
        }
    }
}

fn make_partial_goal_part_1(c: &char) -> State {
    let numpad = parse_numpad(c);
    State {
        numpad,
        directional_pads: vec![DirPad::A, DirPad::A],
        pressed_numpad_button: true,
    }
}

// Returns a vec of partial goal states from a code
fn partial_goals(code: &str) -> Vec<State> {
    vec![make_partial_goal_part_1(&'A')]
        .into_iter()
        .chain(code.chars().map(|c| make_partial_goal_part_1(&c)))
        .collect()
}

// // Assume going from top left to bottom right
// fn bfs(board: &Board) -> Option<u32> {
//     let mut visited: HashSet<Coord> = HashSet::new();
//     let mut predecessors: HashMap<Coord, Coord> = HashMap::new();
//     let mut queue: VecDeque<Coord> = VecDeque::new();
//     let start: Coord = (0, 0);
//     let goal: Coord = (board.rows - 1, board.cols - 1);
//     queue.push_back(start);
//     visited.insert(start);

//     while let Some(coord) = queue.pop_front() {
//         if coord == goal {
//             return Some(backtrack_steps(&predecessors, start, coord));
//         }

//         let neighbors = neighbor_coordinates(&coord, board.rows, board.cols)
//             .into_iter()
//             .filter(|candidate| !visited.contains(candidate))
//             .filter(|candidate| !board.occupied.contains(candidate))
//             .collect_vec();

//         for neigh in neighbors {
//             visited.insert(neigh.clone());
//             predecessors.insert(neigh.clone(), coord);
//             queue.push_back(neigh.clone());
//         }
//     }

//     None // No solution
// }

fn backtrack_steps(predecessors: &HashMap<State, State>, start: State, current: State) -> u64 {
    let predecessor = predecessors
        .get(&current)
        .expect("Should have predecessor!")
        .clone();

    if predecessor == start {
        return 1;
    } else {
        return 1 + backtrack_steps(predecessors, start, predecessor);
    }
}

fn bfs(start: State, goal: State) -> u64 {
    println!("bfs - start: {:?} goal: {:?}", start, goal);

    let mut visited: HashSet<State> = HashSet::new();
    let mut predecessors: HashMap<State, State> = HashMap::new();
    let mut queue: VecDeque<State> = VecDeque::new();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(state) = queue.pop_front() {
        println!("visiting state: {:?}", state.clone());
        if state == goal {
            return backtrack_steps(&predecessors, start, state);
        }

        let neighbors = dirpad_values
            .iter()
            .map(|human_command| do_command(&state, human_command))
            .filter_map(|maybe_next_state| maybe_next_state) // Filters out illegal moves
            .filter(|next_state| !visited.contains(next_state))
            .collect_vec();

        dbg!(neighbors.clone());

        for neigh in neighbors {
            visited.insert(neigh.clone());
            predecessors.insert(neigh.clone(), state.clone());
            queue.push_back(neigh.clone());
        }
    }

    panic!("Couldn't find solution");
}

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

fn solve_single(code: &str) -> u64 {
    let partial_goal_states = partial_goals(code);

    let moves: u64 = partial_goal_states
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(start, goal)| bfs(start.clone(), goal.clone()))
        .sum();

    let numeric_part: u64 = code[0..3].parse().expect("Should parse numeric part!");

    let complexity = moves * numeric_part;
    complexity
}

pub fn part_one(input: &str) -> Option<u64> {
    let codes = input.lines().collect_vec();

    let total_complexity = codes.iter().map(|code| solve_single(code)).sum();

    Some(total_complexity)
}

fn optimal_numpad_parent_pushes(from: NumPad, to: NumPad) -> Vec<DirPad> {
    let mut solution = vec![];
    let from_coord = coord_from_numpad
        .get(&from)
        .expect("should find coord coord from numpad");
    let target_coord = coord_from_numpad
        .get(&to)
        .expect("should find coord coord from numpad");

    let vertical_dir = if target_coord.0 < from_coord.0 {
        DirPad::Up
    } else {
        DirPad::Down
    };
    let vertical_abs_diff = target_coord.0.abs_diff(from_coord.0);

    let horizontal_dir = if target_coord.1 < from_coord.1 {
        DirPad::Left
    } else {
        DirPad::Right
    };
    let horizontal_abs_diff = target_coord.1.abs_diff(from_coord.1);

    match from {
        NumPad::Seven | NumPad::Four | NumPad::One => {
            match to {
                NumPad::One
                | NumPad::Two
                | NumPad::Three
                | NumPad::Four
                | NumPad::Five
                | NumPad::Six
                | NumPad::Seven
                | NumPad::Eight
                | NumPad::Nine => {
                    // Left column ending in middle
                    // Go vertically then horizontally
                    for _ in 0..vertical_abs_diff {
                        solution.push(vertical_dir);
                    }
                    for _ in 0..horizontal_abs_diff {
                        solution.push(horizontal_dir);
                    }
                }
                NumPad::Zero | NumPad::A => {
                    // Left column ending in bottom
                    // Go right then down
                    for _ in 0..horizontal_abs_diff {
                        solution.push(horizontal_dir);
                    }
                    for _ in 0..vertical_abs_diff {
                        solution.push(vertical_dir);
                    }
                }
            }
        }

        NumPad::Two | NumPad::Three | NumPad::Five | NumPad::Six | NumPad::Eight | NumPad::Nine => {
            // Middle or right column.
            if horizontal_dir == DirPad::Left {
                for _ in 0..horizontal_abs_diff {
                    solution.push(horizontal_dir);
                }
                for _ in 0..vertical_abs_diff {
                    solution.push(vertical_dir);
                }
            } else {
                // horizontal dir right, do that last
                for _ in 0..vertical_abs_diff {
                    solution.push(vertical_dir);
                }
                for _ in 0..horizontal_abs_diff {
                    solution.push(horizontal_dir);
                }
            }
        }
        NumPad::Zero | NumPad::A => {
            match to {
                NumPad::One | NumPad::Four | NumPad::Seven => {
                    // Up then left
                    for _ in 0..vertical_abs_diff {
                        solution.push(vertical_dir);
                    }
                    for _ in 0..horizontal_abs_diff {
                        solution.push(horizontal_dir);
                    }
                }
                _ => {
                    if horizontal_dir == DirPad::Left {
                        for _ in 0..horizontal_abs_diff {
                            solution.push(horizontal_dir);
                        }
                        for _ in 0..vertical_abs_diff {
                            solution.push(vertical_dir);
                        }
                    } else {
                        // horizontal dir right, do that last
                        for _ in 0..vertical_abs_diff {
                            solution.push(vertical_dir);
                        }
                        for _ in 0..horizontal_abs_diff {
                            solution.push(horizontal_dir);
                        }
                    }
                }
            }
        }
    }
    solution
}

fn optimal_numpad_parent_path_with_a_suffix(from: NumPad, to: NumPad) -> Vec<DirPad> {
    optimal_numpad_parent_pushes(from, to)
        .into_iter()
        .chain([DirPad::A])
        .collect()
}

fn optimal_numpad_parent_path_with_a_prefix_suffix(from: NumPad, to: NumPad) -> Vec<DirPad> {
    // optimal_numpad_parent_pushes(from, to)
    //     .into_iter()
    //     .chain([DirPad::A])
    //     .collect()
    [DirPad::A]
        .into_iter()
        .chain(optimal_numpad_parent_pushes(from, to))
        .chain([DirPad::A])
        .collect()
}

fn optimal_dirpad_path(from: DirPad, to: DirPad) -> Vec<DirPad> {
    match to {
        DirPad::Up => match from {
            DirPad::Up => vec![],
            DirPad::A => vec![DirPad::Left],
            DirPad::Left => vec![DirPad::Right, DirPad::Up],
            DirPad::Down => vec![DirPad::Up],
            DirPad::Right => vec![DirPad::Left, DirPad::Up],
        },
        DirPad::A => match from {
            DirPad::Up => vec![DirPad::Right],
            DirPad::A => vec![],
            DirPad::Left => vec![DirPad::Right, DirPad::Right, DirPad::Up],
            // DirPad::Down => vec![DirPad::Right, DirPad::Up],
            DirPad::Down => vec![DirPad::Up, DirPad::Right],
            DirPad::Right => vec![DirPad::Up],
        },
        DirPad::Left => match from {
            DirPad::Up => vec![DirPad::Down, DirPad::Left],
            DirPad::A => vec![DirPad::Down, DirPad::Left, DirPad::Left],
            DirPad::Left => vec![],
            DirPad::Down => vec![DirPad::Left],
            DirPad::Right => vec![DirPad::Left, DirPad::Left],
        },
        DirPad::Down => match from {
            DirPad::Up => vec![DirPad::Down],
            // DirPad::A => vec![DirPad::Down, DirPad::Left],
            DirPad::A => vec![DirPad::Left, DirPad::Down],
            DirPad::Left => vec![DirPad::Right],
            DirPad::Down => vec![],
            DirPad::Right => vec![DirPad::Left],
        },
        DirPad::Right => match from {
            DirPad::Up => vec![DirPad::Down, DirPad::Right],
            DirPad::A => vec![DirPad::Down],
            DirPad::Left => vec![DirPad::Right, DirPad::Right],
            DirPad::Down => vec![DirPad::Right],
            DirPad::Right => vec![],
        },
    }
}

fn optimal_dirpad_parent_path_with_a_suffix(from: DirPad, to: DirPad) -> Vec<DirPad> {
    // [DirPad::A].into_iter().chain(
    // optimal_dirpad_path(from, to)
    //     .into_iter()
    //     .chain([DirPad::A]))
    //     .collect()

    optimal_dirpad_path(from, to)
        .into_iter()
        .chain([DirPad::A])
        .collect()
}

fn optimal_dirpad_parent_path_with_a_prefix_suffix(from: DirPad, to: DirPad) -> Vec<DirPad> {
    [DirPad::A]
        .into_iter()
        .chain(optimal_dirpad_path(from, to).into_iter().chain([DirPad::A]))
        .collect()
}

fn pushes_needed(
    cache: &mut HashMap<(usize, Vec<DirPad>), u64>,
    depth: usize,
    desired_sequence: Vec<DirPad>,
) -> u64 {
    if let Some(cached_result) = cache.get(&(depth, desired_sequence.clone())) {
        return *cached_result;
    }

    // If we are at zero depth (human keypad, just press the optimal path)
    // let mut result = 0;

    let result = if depth == 0 {
        desired_sequence.len().try_into().expect("Should convert")
    } else {
        [DirPad::A]
            .iter()
            .chain(desired_sequence.iter())
            .tuple_windows()
            .map(|(next_from, next_to)| move_count_dirpad(cache, *next_from, *next_to, depth))
            .sum()
    };

    cache.insert((depth, desired_sequence), result);

    return result;
}

// Inspired by https://www.reddit.com/r/adventofcode/comments/1hj2odw/comment/m36j01x/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
fn move_count_dirpad(
    cache: &mut HashMap<(usize, Vec<DirPad>), u64>,
    from: DirPad,
    to: DirPad,
    depth: usize,
) -> u64 {
    if from == to {
        return 1;
    }

    let next_sequence = optimal_dirpad_parent_path_with_a_suffix(from, to);
    // let next_sequence = optimal_dirpad_parent_path_with_a_prefix_suffix(from, to);
    pushes_needed(cache, depth - 1, next_sequence)
}

fn move_count_numpad(
    cache: &mut HashMap<(usize, Vec<DirPad>), u64>,
    from: NumPad,
    to: NumPad,
    depth: usize,
) -> u64 {
    if from == to {
        return 1;
    }

    // let next_sequence = optimal_numpad_parent_path_with_a_suffix(from, to);
    let next_sequence = optimal_numpad_parent_path_with_a_suffix(from, to);
    // let next_sequence = optimal_numpad_parent_path_with_a_prefix_suffix(from, to);
    pushes_needed(cache, depth, next_sequence) // Keep depth
}

fn get_button_presses(code: &str, max_depth: usize, cache: &mut HashMap<(usize, Vec<DirPad>), u64>) -> u64 {
    let prepended_code: Vec<char> = ['A'].into_iter().chain(code.chars()).collect();
    let button_presses: u64 = prepended_code
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(from_numpad_char, to_numpad_char)| {
            let from_numpad = parse_numpad(from_numpad_char);
            let to_numpad = parse_numpad(to_numpad_char);
            let button_presses: u64 = move_count_numpad(cache, from_numpad, to_numpad, max_depth);
            button_presses
        })
        .sum();
    button_presses
}

fn solve_single_part_2(
    code: &str,
    max_depth: usize,
    cache: &mut HashMap<(usize, Vec<DirPad>), u64>,
) -> u64 {

    let button_presses = get_button_presses(code, max_depth, cache);
    println!("code: {}, button presses: {}", code, button_presses);

    let numeric_part: u64 = code[0..3].parse().expect("Should parse numeric part!");
    let complexity: u64 = button_presses * numeric_part;

    // 0
    complexity
}

fn empty_cache() -> HashMap<(usize, Vec<DirPad>), u64> {
    HashMap::new()
}

fn solve_all_part_2(input: &str, max_depth: usize) -> Option<u64> {
    let codes = input.lines().collect_vec();

    // let mut cache: HashMap<(usize, DirPad, DirPad), u64> = HashMap::new();
    let mut cache = empty_cache();

    let total_complexity = codes
        .iter()
        .map(|code| solve_single_part_2(code, max_depth, &mut cache))
        .sum();

    // for (k, v) in cache {
    //     println!("key: {:?}, value: {}", k, v);
    // }

    Some(total_complexity)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Idea: Bottom-up solution
    // TODO: Create map of optimal dirpad presses to get between any two numpad values

    // Want to recursively find the number of human presses we need to do a move at our current depth
    // The key idea here is that since we know every robot arm that is more shallow needs to be at A
    // to end up in our situation, we can cache these.

    solve_all_part_2(input, 25)
    // 268750622157767 too high
}

/*
Debugging:
key: (2, Right, A), value: 4 - Seems fine
key: (1, Right, Up), value: 5
key: (1, Left, Left), value: 0
key: (1, Down, Left), value: 4
key: (1, Left, Right), value: 3
key: (2, Down, Left), value: 10 - this one is wrong (should be 8) - why?
key: (1, Right, Right), value: 0
key: (2, Left, A), value: 14
key: (1, Down, Up), value: 2
key: (2, Down, Down), value: 10
key: (2, Up, Up), value: 6
key: (2, Left, Left), value: 10
key: (1, Up, A), value: 2
key: (2, Down, Right), value: 9
key: (2, Right, Right), value: 5
key: (1, Left, A), value: 6
key: (1, Right, A), value: 2
key: (2, Up, A), value: 5
key: (1, Down, A), value: 5
key: (2, Down, A), value: 14
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = solve_all_part_2(&advent_of_code::template::read_file("examples", DAY), 2);
        // let result = solve_all_part_2(&advent_of_code::template::read_file("examples", DAY), 3);
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two_depth_0() {
        let result = get_button_presses("029A", 0, &mut empty_cache());
        assert_eq!(result, "<A^A>^^AvvvA".len().try_into().unwrap());
    }


    #[test]
    fn test_part_two_first_presses() {
        let result = get_button_presses("029A", 2, &mut empty_cache());
        assert_eq!(result, 68);
    }

    #[test]
    fn test_part_two_second_presses() {
        let result = get_button_presses("980A", 2, &mut empty_cache());
        assert_eq!(result, 60);
    }

    #[test]
    fn test_part_two_third_presses() {
        let result = get_button_presses("179A", 2, &mut empty_cache());
        assert_eq!(result, 68);
    }

    #[test]
    fn test_part_two_fourth_presses() {
        let result = get_button_presses("456A", 2, &mut empty_cache());
        assert_eq!(result, 64);
    }

    #[test]
    fn test_part_two_fifth_presses() {
        let result = get_button_presses("379A", 2, &mut empty_cache());
        assert_eq!(result, 64);
    }


    #[test]
    fn test_part_two_first() {
        let result = solve_all_part_2("029A", 2);
        assert_eq!(result, Some(68 * 29));
    }

    #[test]
    fn test_part_two_second() {
        let result = solve_all_part_2("980A", 2);
        assert_eq!(result, Some(60 * 980));
    }


    #[test]
    fn test_part_two_third() {
        let result = solve_all_part_2("179A", 2);
        assert_eq!(result, Some(68 * 179));
    }

    #[test]
    fn test_part_two_fourth() {
        let result = solve_all_part_2("456A", 2);
        assert_eq!(result, Some(64 * 456));
    }
    #[test]

    fn test_part_two_fifth() {
        let result = solve_all_part_2("379A", 2);
        assert_eq!(result, Some(64 * 379));
    }
}
