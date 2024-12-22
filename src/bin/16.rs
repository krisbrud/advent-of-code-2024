use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
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

fn djikstra_shortest_path_part_1(board: Board) -> Option<usize> {
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
    let goal_coord = find_needle_or_fail(&board, GOAL);
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

fn backtrack_all(
    costs_and_predecessors: &HashMap<Pose, (usize, Vec<Pose>)>,
    pose: Pose,
) -> Vec<Pose> {
    if let Some((_, predecessors)) = costs_and_predecessors.get(&pose) {
        let mut out: Vec<Pose> = predecessors
            .iter()
            .flat_map(|predecessor| backtrack_all(costs_and_predecessors, *predecessor))
            .collect();

        out.push(pose);

        return out;
    } else {
        return vec![];
    }
}

fn djikstra_best_seats_part_2(board: Board) -> Option<HashSet<Coord>> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut pose_cost_and_predecessors: HashMap<Pose, (usize, Vec<Pose>)> = HashMap::new();

    let mut heap = BinaryHeap::new();

    let start_coord = find_needle_or_fail(&board, START);
    let start_pose = Pose {
        coord: start_coord,
        direction: Direction::East, // Always the case
    };
    let goal_coord = find_needle_or_fail(&board, GOAL);
    let goals: Vec<Pose> = ALL_DIRECTIONS
        .iter()
        .map(|dir| Pose {
            coord: goal_coord,
            direction: *dir,
        })
        .collect_vec();

    // We're at `start`, with a zero cost
    pose_cost_and_predecessors.insert(start_pose, (0, vec![]));
    heap.push(State {
        pose: start_pose,
        cost: 0,
    });

    let mut found_goal = false;

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { pose, cost }) = heap.pop() {
        // dbg!(pose.clone(), cost.clone());

        // Alternatively we could have continued to find all shortest paths
        if goals.contains(&pose) {
            // let all_coords = backtrack_all(pose_cost_and_predecessors, goal_coord);
            // let unique_coords: HashSet<Coord> = all_coords.into_iter().collect();
            // return Some(unique_coords);
            println!("found goal");

            found_goal = true;

            continue;
        }

        // Important as we may have already found a better way
        let (pose_cost, _) = pose_cost_and_predecessors[&pose];
        if cost > pose_cost {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (adj_pose, additional_cost) in adjacent_poses_and_additional_costs(&board, &pose) {
            let next = State {
                pose: adj_pose,
                cost: cost + additional_cost,
            };

            // When exploring a new tile - insert into the hashmap
            // When exploring a previous tile:
            //   if the cost is better than the previous best, make a new vec with the pose and the cost
            //   if the cost is the same -> append the pose and cost

            // let (adj_pose_curr_best_cost, adj_pose_predecessors) =
            if let Some((adj_pose_curr_best_cost, adj_pose_predecessors)) =
                pose_cost_and_predecessors.get(&adj_pose)
            {
                if next.cost == *adj_pose_curr_best_cost {
                    // Append and insert
                    let mut new_adj_pose_predecessors = adj_pose_predecessors.clone();
                    new_adj_pose_predecessors.push(pose);
                    pose_cost_and_predecessors.insert(
                        adj_pose,
                        (next.cost, new_adj_pose_predecessors),
                    );

                    // No need to explore again by pushing to heap
                } else if next.cost < *adj_pose_curr_best_cost {
                    heap.push(next);
                    // Relaxation, we have now found a better way
                    pose_cost_and_predecessors.insert(next.pose, (next.cost, vec![pose]));
                }
            } else {
                heap.push(next);
                // Relaxation, we have now found a better way
                pose_cost_and_predecessors.insert(next.pose, (next.cost, vec![pose]));
            }
        }
    }

    if found_goal {
        let mut best_cost = usize::MAX;
        for direction in ALL_DIRECTIONS {
            if let Some(tup) = pose_cost_and_predecessors.get(&Pose{ coord: goal_coord, direction: *direction}) {
                let (cost, _) = tup;
                if *cost < best_cost {
                    best_cost = *cost;
                }
            }
        }

        let mut goal_poses: Vec<Pose> = vec![];
        for direction in ALL_DIRECTIONS {
            let possible_pose = Pose{ coord: goal_coord, direction: *direction};
            if let Some(tup) = pose_cost_and_predecessors.get(&possible_pose) {
                let (cost, _) = tup;
                if *cost == best_cost {
                    goal_poses.push(possible_pose);
                }
            }
        }

        let all_poses = goal_poses.iter().flat_map(|p| {
            backtrack_all(
                &pose_cost_and_predecessors,
                *p
            )
        });
        let unique_coords: HashSet<Coord> = all_poses
            .into_iter()
            .map(|some_pose| some_pose.coord)
            .collect();
        return Some(unique_coords);
    }

    // Goal not reachable
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let board = Board::new(input).expect("Board should parse!");

    let shortest_path_cost =
        djikstra_shortest_path_part_1(board).expect("Should find shortest path!");
    Some(shortest_path_cost.try_into().ok()?)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Idea: Modify implementation from part 1 by
    // Make a HashMap predecessors from pose to Vec<(prev_pose, cost)>. Use this to mark that a tile is visited with the best cost so far. When doing this we can replace the pose_cost HashMap
    // When exploring a new tile - insert into the hashmap
    // When exploring a previous tile:
    //   if the cost is better than the previous best, make a new vec with the pose and the cost
    //   if the cost is the same -> append the pose and cost
    let board = Board::new(input).expect("Board should parse!");

    let unique_coords = djikstra_best_seats_part_2(board).expect("Should find shortest path!");
    println!("unique coords: {:?}", unique_coords);
    Some(unique_coords.len().try_into().ok()?)

    // None
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

    #[test]
    fn test_part_two_1() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(45));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
