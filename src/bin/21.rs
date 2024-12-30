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

fn make_partial_goal_part_1(c: &char) -> State {
    let numpad = match c {
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
    };

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

// Returns complexity
fn solve_single(code: &str) -> u64 {
    let partial_goal_states = partial_goals(code);

    let moves: u64 = partial_goal_states.iter().tuple_windows::<(_,_)>()
        .map(|(start, goal)| {
            bfs(start.clone(), goal.clone())
        }).sum();

    let numeric_part: u64 = code[0..3].parse().expect("Should parse numeric part!");

    let complexity = moves * numeric_part;
    complexity
}

pub fn part_one(input: &str) -> Option<u64> {
    let codes = input.lines().collect_vec();

    let total_complexity = codes.iter().map(|code| solve_single(code)).sum();

    Some(total_complexity)
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
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
