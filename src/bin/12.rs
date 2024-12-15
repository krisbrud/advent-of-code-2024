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

#[derive(PartialEq, Debug)]
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
        if region.get(&(coordinate.0 - 1, coordinate.1)).is_none() {
            side_pieces.push((coordinate.clone(), Direction::North))
        }
    }
    if coordinate.1 == rows - 1 {
        side_pieces.push((coordinate.clone(), Direction::South))
    } else {
        if region.get(&(coordinate.0 + 1, coordinate.1)).is_none() {
            side_pieces.push((coordinate.clone(), Direction::South))
        }
    }
    if coordinate.1 == 0 {
        side_pieces.push((coordinate.clone(), Direction::West))
    } else {
        if region.get(&(coordinate.0, coordinate.1 - 1)).is_none() {
            side_pieces.push((coordinate.clone(), Direction::West))
        }
    }
    if coordinate.1 == cols - 1 {
        side_pieces.push((coordinate.clone(), Direction::East))
    } else {
        if region.get(&(coordinate.0, coordinate.1 + 1)).is_none() {
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

fn sides_horizontally(side_pieces: Vec<Coordinate>) -> u32 {
    // North and South-facing sides
    let row_groups = side_pieces.iter().into_group_map_by(|(row, _)| row);

    let mut sides = 0;
    for (_, side_pieces) in row_groups {
        let mut pieces = side_pieces.clone();
        pieces.sort_by_key(|(row, col)| col);
        let tuples = pieces
            .into_iter()
            .map(|(_, col)| col)
            .tuple_windows::<(_, _)>();

        sides += 1; // the calculation below only counts the gaps

        for (first, second) in tuples {
            if *first + 1 != *second {
                sides += 1;
            }
        }
    }

    sides
}

fn sides_vertically(side_pieces: Vec<Coordinate>) -> u32 {
    // North and South-facing sides
    let col_groups = side_pieces.iter().into_group_map_by(|(_, col)| col);

    let mut sides = 0;
    for (_, side_pieces) in col_groups {
        let mut pieces = side_pieces.clone();
        pieces.sort_by_key(|(row, _)| row);
        let tuples = pieces
            .into_iter()
            .map(|(row, _)| row)
            .tuple_windows::<(_, _)>();

        sides += 1; // the calculation below only counts the gaps

        for (first, second) in tuples {
            if *first + 1 != *second {
                sides += 1;
            }
        }
    }

    sides
}

fn region_sides(region: &HashSet<Coordinate>, rows: usize, cols: usize) -> u32 {
    let side_pieces = region_side_pieces(region, rows, cols);
    println!("region_sides region {:?}, side_pieces: {:?}, num side pieces: {}", region, side_pieces, side_pieces.len());

    let north_pieces = side_pieces
        .iter()
        .filter(|(_, dir)| dir == &Direction::North)
        .map(|(coord, _)| *coord)
        .collect_vec();
    let south_pieces = side_pieces
        .iter()
        .filter(|(_, dir)| dir == &Direction::South)
        .map(|(coord, _)| *coord)
        .collect_vec();
    let east_pieces = side_pieces
        .iter()
        .filter(|(_, dir)| dir == &Direction::East)
        .map(|(coord, _)| *coord)
        .collect_vec();
    let west_pieces = side_pieces
        .iter()
        .filter(|(_, dir)| dir == &Direction::West)
        .map(|(coord, _)| *coord)
        .collect_vec();

    let north_sides = sides_horizontally(north_pieces);
    let south_sides = sides_horizontally(south_pieces);
    let west_sides = sides_vertically(west_pieces);
    let east_sides = sides_horizontally(east_pieces);

    north_sides + south_sides + west_sides + east_sides
}

fn price_part_2(region: &HashSet<Coordinate>, rows: usize, cols: usize) -> u32 {
    let area_ = area(region);
    let sides = region_sides(region, rows, cols);
    area_ * sides
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

    let total_price: u32 = regions
        .iter()
        .map(|region| price_part_2(region, farm.rows, farm.cols))
        .sum();

    Some(total_price)
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
    fn test_sides_horizontally() {
        assert_eq!(2, sides_horizontally(vec![(0, 0), (0, 2), (0, 3)]));
        assert_eq!(
            4,
            sides_horizontally(vec![(0, 0), (0, 2), (0, 3), (1, 3), (1, 5)])
        );
    }

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

    #[test]
    fn test_part_two_first_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_second_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two_third_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_fourth_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(236));
    }

    #[test]
    fn test_part_two_fifth_example() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(368));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
