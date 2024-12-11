use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(10);

type Coordinate = (i32, i32);
type Terrain = HashMap<Coordinate, i32>;

fn parse(s: &str) -> Option<Terrain> {
    let terrain: Terrain = s
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            let out = line.char_indices().map(move |(col, tile)| {
                if let Some(height) = tile.to_string().parse::<u32>().ok() {
                    Some(((row as i32, col as i32), height as i32))
                } else {
                    None
                }
            });
            out
        })
        .collect::<Option<Terrain>>()?;

    Some(terrain)
}

fn is_within_bounds(coordinate: &Coordinate, rows: i32, cols: i32) -> bool {
    return (0 <= coordinate.0)
        && (coordinate.0 < rows)
        && (0 <= coordinate.1)
        && (coordinate.1 < cols);
}

fn neighbor_coordinates(coordinate: &Coordinate, rows: i32, cols: i32) -> Vec<Coordinate> {
    vec![
        (coordinate.0 + 1, coordinate.1),
        (coordinate.0 - 1, coordinate.1),
        (coordinate.0, coordinate.1 + 1),
        (coordinate.0, coordinate.1 - 1),
    ]
    .into_iter()
    .filter(|c| is_within_bounds(coordinate, rows, cols))
    .collect()
}

fn neighbors(
    terrain: &Terrain,
    coordinate: &Coordinate,
    rows: i32,
    cols: i32,
) -> Vec<(Coordinate, i32)> {
    neighbor_coordinates(coordinate, rows, cols)
        .into_iter()
        .map(|coordinate| {
            if let Some(height) = terrain.get(&coordinate) {
                Some((coordinate, *height))
            } else {
                None
            }
        })
        .filter_map(|x| x)
        .collect()
}

fn reachable_tops(
    terrain: &Terrain,
    coordinate: &Coordinate,
    rows: i32,
    cols: i32,
) -> HashSet<Coordinate> {
    let height = terrain.get(coordinate).unwrap();

    if *height == 9 {
        return vec![coordinate.clone()].into_iter().collect::<HashSet<_>>();
    }

    let neighbor_tiles = neighbors(terrain, coordinate, rows, cols);

    let possible_next_tiles = neighbor_tiles
        .iter()
        .filter(|(_, neigh_height)| *neigh_height == (height + 1));

    let reachable: HashSet<Coordinate> = possible_next_tiles
        .flat_map(|(neigh_coord, _)| reachable_tops(terrain, neigh_coord, rows, cols))
        .collect();

    reachable
}

fn reachable_top_paths(terrain: &Terrain, coordinate: &Coordinate, rows: i32, cols: i32) -> u32 {
    let height = terrain.get(coordinate).unwrap();

    if *height == 9 {
        return 1;
    }

    let neighbor_tiles = neighbors(terrain, coordinate, rows, cols);

    let possible_next_tiles = neighbor_tiles
        .iter()
        .filter(|(_, neigh_height)| *neigh_height == (height + 1));

    let score = possible_next_tiles
        .map(|(neigh_coord, _)| reachable_top_paths(terrain, neigh_coord, rows, cols))
        .sum();
    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let terrain: Terrain = parse(input)?;

    let rows: i32 = input.lines().collect_vec().len().try_into().ok()?;
    let cols: i32 = input.lines().nth(0).map(|x| x.len())?.try_into().ok()?;

    let start_positions: Vec<Coordinate> = terrain
        .clone()
        .into_iter()
        .filter(|(_, height)| *height == 0)
        .map(|(coord, _)| coord)
        .collect();

    let total_score = start_positions
        .iter()
        .map(|start_coord| {
            let reachable = reachable_tops(&terrain, start_coord, rows, cols);
            let count = reachable.len();
            count as u32
        }).sum();

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let terrain: Terrain = parse(input)?;

    let rows: i32 = input.lines().collect_vec().len().try_into().ok()?;
    let cols: i32 = input.lines().nth(0).map(|x| x.len())?.try_into().ok()?;

    let start_positions: Vec<Coordinate> = terrain
        .clone()
        .into_iter()
        .filter(|(_, height)| *height == 0)
        .map(|(coord, _)| coord)
        .collect();

    let total_score = start_positions
        .iter()
        .map(|start_coord| {
            let reachable_paths = reachable_top_paths(&terrain, start_coord, rows, cols);
            reachable_paths as u32
        }).sum();

    Some(total_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
