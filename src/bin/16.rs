use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    os::macos::raw::stat,
    usize,
};

use itertools::Itertools;

advent_of_code::solution!(16);

// Implementation partly based on rust binary heap docs: https://doc.rust-lang.org/nightly/std/collections/binary_heap/index.html

type Coord = (usize, usize);

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn possible_rotations(&self) -> Vec<Direction> {
        match self {
            Direction::North | Direction::South => vec![Direction::West, Direction::East],
            Direction::East | Direction::West => vec![Direction::North, Direction::South],
        }
    }
}

// Coordinates in format (y, x)
fn coord_in_direction(coord: Coord, direction: Direction) -> Option<Coord> {
    match direction {
        Direction::North => Some((coord.0.checked_sub(1)?, coord.1)),
        Direction::South => Some((coord.0 + 1, coord.1)),
        Direction::East => Some((coord.0, coord.1 + 1)),
        Direction::West => Some((coord.0, coord.1.checked_sub(1)?)),
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pose {
    coord: Coord,
    direction: Direction,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    pose: Pose,
    cost: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.pose.coord.cmp(&other.pose.coord))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Possible tiles
const WALL: char = '#';
const AIR: char = '.';
const START: char = 'S';
const GOAL: char = 'E';

struct Board {
    tiles: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn new(s: &str) -> Option<Board> {
        let plants: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let rows = s.lines().collect_vec().len();
        let cols = s.lines().nth(0).map(|x| x.len())?;

        Some(Board {
            tiles: plants,
            rows,
            cols,
        })
    }

    // Assume bounds check has already been done
    fn get(&self, coordinate: &Coord) -> char {
        self.tiles[coordinate.0][coordinate.1]
    }
}

// (pose, additional_cost)
fn adjacent_poses_and_additional_costs(board: &Board, pose: &Pose) -> Vec<(Pose, usize)> {
    let mut adj: Vec<(Pose, usize)> = vec![];

    // Straight ahead
    if let Some(front_coord) = coord_in_direction(pose.coord, pose.direction) {
        // Check if there is a wall there
        if board.get(&front_coord) != WALL {
            adj.push((
                Pose {
                    coord: front_coord,
                    direction: pose.direction,
                },
                1,
            ));
        }
    }

    // Turning is always possible
    for turned_dir in pose.direction.possible_rotations() {
        adj.push((
            Pose {
                coord: pose.coord,
                direction: turned_dir,
            },
            1000,
        ));
    }

    adj
}

static ALL_DIRECTIONS: &'static [Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

fn find_needle_or_fail(board: &Board, needle: char) -> Coord {
    for (i, row) in board.tiles.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if *tile == needle {
                return (i, j);
            }
        }
    }

    panic!("Couldn't find tile!");
}

fn djikstra_shortest_path(board: Board) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut pose_cost: HashMap<Pose, usize> = board
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(row_idx, row_tiles)| {
            row_tiles.iter().enumerate().flat_map(move |(col_idx, _)| {
                let coord = (row_idx, col_idx);
                let cost = usize::MAX;
                ALL_DIRECTIONS.iter().map(move |direction| {
                    (
                        Pose {
                            coord,
                            direction: *direction,
                        },
                        cost,
                    )
                })
            })
        })
        .collect();

    let mut heap = BinaryHeap::new();

    let start_coord = find_needle_or_fail(&board, START);
    let start_pose = Pose {
        coord: start_coord,
        direction: Direction::East, // Always the case
    };
    dbg!(start_coord);
    let goal_coord = find_needle_or_fail(&board, GOAL);
    dbg!(goal_coord);
    let goals: Vec<Pose> = ALL_DIRECTIONS
        .iter()
        .map(|dir| Pose {
            coord: goal_coord,
            direction: *dir,
        })
        .collect_vec();

    // We're at `start`, with a zero cost
    pose_cost.insert(start_pose, 0);
    heap.push(State {
        pose: start_pose,
        cost: 0,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { pose, cost }) = heap.pop() {
        // dbg!(pose.clone(), cost.clone());

        // Alternatively we could have continued to find all shortest paths
        if goals.contains(&pose) {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > pose_cost[&pose] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (adj_pose, additional_cost) in adjacent_poses_and_additional_costs(&board, &pose) {
            let next = State {
                pose: adj_pose,
                cost: cost + additional_cost,
            };

            // If so, add it to the frontier and continue
            if next.cost < pose_cost[&next.pose] {
                heap.push(next);
                // Relaxation, we have now found a better way
                pose_cost.insert(next.pose, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
// fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
//     // dist[node] = current shortest distance from `start` to `node`
//     let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

//     let mut heap = BinaryHeap::new();

//     // We're at `start`, with a zero cost
//     dist[start] = 0;
//     heap.push(State { cost: 0, position: start });

//     // Examine the frontier with lower cost nodes first (min-heap)
//     while let Some(State { cost, position }) = heap.pop() {
//         // Alternatively we could have continued to find all shortest paths
//         if position == goal { return Some(cost); }

//         // Important as we may have already found a better way
//         if cost > dist[position] { continue; }

//         // For each node we can reach, see if we can find a way with
//         // a lower cost going through this node
//         for edge in &adj_list[position] {
//             let next = State { cost: cost + edge.cost, position: edge.node };

//             // If so, add it to the frontier and continue
//             if next.cost < dist[next.position] {
//                 heap.push(next);
//                 // Relaxation, we have now found a better way
//                 dist[next.position] = next.cost;
//             }
//         }
//     }

//     // Goal not reachable
//     None
// }

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::new(input).expect("Board should parse!");

    let shortest_path_cost = djikstra_shortest_path(board).expect("Should find shortest path!");
    Some(shortest_path_cost.try_into().ok()?)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
