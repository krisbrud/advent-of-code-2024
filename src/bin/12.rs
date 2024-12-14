use core::num;
use std::{collections::HashSet, vec};

use itertools::Itertools;

advent_of_code::solution!(12);

type Coordinate = (usize, usize);

struct Farm {
    plants: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Farm {
    fn new(s: &str) -> Option<Farm> {
        let plants: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

        let rows = s.lines().collect_vec().len();
        let cols = s.lines().nth(0).map(|x| x.len())?;

        Some(Farm { plants, rows, cols })
    }
}

fn neighbor_coordinates(coord: &Coordinate, rows: usize, cols: usize) -> Vec<Coordinate> {
    let mut neightbors: Vec<Coordinate> = vec![];

    if coord.0 > 0 {
        neightbors.push((coord.0 - 1, coord.1));
    }

    if coord.0 < cols - 1 {
        neightbors.push((coord.0 + 1, coord.1));
    }

    if coord.1 > 0 {
        neightbors.push((coord.0, coord.1 - 1));
    }

    if coord.1 < rows - 1 {
        neightbors.push((coord.0, coord.1 + 1));
    }

    neightbors
}

fn neighbors_in_same_region(farm: &Farm, coordinate: &Coordinate) -> Vec<Coordinate> {
    let region_char = farm.plants[coordinate.0][coordinate.1];

    neighbor_coordinates(coordinate, farm.rows, farm.cols)
        .into_iter()
        .filter(|coord| farm.plants[coord.0][coord.1] == region_char)
        .collect()
}

fn find_regions(farm: &Farm) -> Vec<HashSet<Coordinate>> {
    println!("Start of find regions");
    // General idea:
    // Make a set of all the possible coordinates
    let mut remaining_coordinates: HashSet<Coordinate> = (0..farm.rows)
        .flat_map(|row| (0..farm.cols).map(move |col| (row, col)))
        .collect();

    let mut regions: Vec<HashSet<Coordinate>> = vec![];

    while !remaining_coordinates.is_empty() {
        let starting_point = remaining_coordinates.iter().next().unwrap().clone();
        remaining_coordinates.remove(&starting_point);

        let mut neighbors_to_explore: HashSet<Coordinate> = HashSet::new();
        neighbors_to_explore.insert(starting_point);

        let mut region: HashSet<Coordinate> = HashSet::new();
        region.insert(starting_point);
        while !neighbors_to_explore.is_empty() {
            let coord = neighbors_to_explore.iter().next().unwrap().clone();
            let removed = neighbors_to_explore.remove(&coord);

            for neighbor in neighbors_in_same_region(farm, &coord) {
                let existing = region.get(&neighbor).copied();
                region.insert(neighbor);

                if existing.is_none() {
                    // Not yet explored from
                    neighbors_to_explore.insert(neighbor);
                }
            }
        }

        regions.push(region.clone());
        remaining_coordinates = remaining_coordinates.difference(&region).copied().collect();
    }

    regions
}

fn area(region: &HashSet<Coordinate>) -> u32 {
    region.len().try_into().unwrap()
}

fn perimeter(region: &HashSet<Coordinate>, rows: usize, cols: usize) -> u32 {
    region
        .iter()
        .map(|coordinate| {
            let neighbors = neighbor_coordinates(coordinate, rows, cols);
            let num_same_region: u32 = region
                .intersection(&neighbors.into_iter().collect::<HashSet<Coordinate>>())
                .collect::<HashSet<_>>()
                .len()
                .try_into()
                .unwrap();
            4 - num_same_region
        })
        .sum()
}

fn plant_side_pieces(
    region: &HashSet<Coordinate>,
    coordinate: &Coordinate,
    rows: usize,
    cols: usize,
) -> Vec<(Coordinate, Direction)> {
    let mut side_pieces: Vec<(Coordinate, Direction)> = vec![];
    if coordinate.0 == 0 {
        side_pieces.push((coordinate.clone(), Direction::North))
    } else {
        if region.get(&(coordinate.0 - 1, coordinate.1)).is_some() {
            side_pieces.push((coordinate.clone(), Direction::North))
        }
    }
    if coordinate.1 == rows - 1 {
        side_pieces.push((coordinate.clone(), Direction::South))
    } else {
        if region.get(&(coordinate.0 + 1, coordinate.1)).is_some() {
            side_pieces.push((coordinate.clone(), Direction::South))
        }
    }
    if coordinate.1 == 0 {
        side_pieces.push((coordinate.clone(), Direction::West))
    } else {
        if region.get(&(coordinate.0, coordinate.1 - 1)).is_some() {
            side_pieces.push((coordinate.clone(), Direction::West))
        }
    }
    if coordinate.1 == cols - 1 {
        side_pieces.push((coordinate.clone(), Direction::East))
    } else {
        if region.get(&(coordinate.0, coordinate.1 + 1)).is_some() {
            side_pieces.push((coordinate.clone(), Direction::East))
        }
    }

    side_pieces
}

fn region_side_pieces(
    region: &HashSet<Coordinate>,
    rows: usize,
    cols: usize,
) -> Vec<(Coordinate, Direction)> {
    region
        .into_iter()
        .flat_map(|coordinate| plant_side_pieces(region, coordinate, rows, cols))
        .collect()
}

fn region_sides(region: &HashSet<Coordinate>, rows: usize, cols: usize) -> u32 {
    let side_pieces = region_side_pieces(region, rows, cols);

    let north_pieces = side_pieces
        .iter()
        .filter(|(_, dir)| dir == &Direction::North);

    let south_pieces = side_pieces
        .iter()
        .filter(|(_, dir)| dir == &Direction::South);
    let east_pieces = side_pieces
        .iter()
        .filter(|(_, dir)| dir == &Direction::East);
    let west_pieces = side_pieces
        .iter()
        .filter(|(_, dir)| dir == &Direction::West);

    0
}

fn price_part_2(region: &HashSet<Coordinate>, rows: usize, cols: usize) -> u32 {
    area(region) * perimeter(region, rows, cols)
}

fn price_part_1(region: &HashSet<Coordinate>, rows: usize, cols: usize) -> u32 {
    area(region) * perimeter(region, rows, cols)
}

pub fn part_one(input: &str) -> Option<u32> {
    let farm = Farm::new(input)?;
    let regions = find_regions(&farm);

    let total_price: u32 = regions
        .iter()
        .map(|region| price_part_1(region, farm.rows, farm.cols))
        .sum();

    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    // The price is now for the number of sides per region instead of the perimeter.
    // We can change the perimeter code to instead of taking 4 - the available sides,
    // it should return the positions and directions in which there is no neighbor in the same region.
    // We will then have sequences of tuples (coordinate, direction).
    // This should be split up in sides according to the rules:
    // A west/east side must have the same col coordinate and contiguous row coordinates
    // A north/south side must have the same row coordinate and contiguous col coordinates

    let farm = Farm::new(input)?;
    let regions = find_regions(&farm);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_find_regions() {
    //     let farm = Farm::new(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 1,
    //     ))
    //     .unwrap();
    //     let actual = find_regions(&farm);
    //     let expected: Vec<HashSet<Coordinate>> = vec![];
    //     assert_eq!(expected, actual);
    // }

    #[test]
    fn test_part_one_first_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_second_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_one_third_example() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1930));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
