advent_of_code::solution!(4);

fn num_cols(input: &str) -> usize {
    let first_line = input.lines().next().unwrap();
    first_line.len() 
}

fn num_rows(input: &str) -> usize {
    let lines: Vec<&str> = input.lines().collect();
    lines.len()
}

fn vertical_candidates(input: &str) -> Vec<String> {
    let rows = num_rows(input);
    let cols = num_cols(input);

    let mut candidates: Vec<String> = vec![];

    let lines: Vec<&str> = input.lines().collect();

    for i_col in 0..cols {
        for i_start_row in 0..rows-3 {
            let mut candidate = String::new();
            for i_row in i_start_row..i_start_row+4 {
                let character = lines[i_row].chars().nth(i_col).unwrap();
                candidate.push(character);
            }
            candidates.push(candidate)
        }
    }

    candidates
}

fn horizontal_candidates(input: &str) -> Vec<&str> {
    let rows = num_rows(input);
    let cols = num_cols(input);

    let mut candidates: Vec<&str> = vec![];

    let lines: Vec<&str> = input.lines().collect();

    for i_row in 0..rows {
        let line = lines[i_row];
        for i_col in 0..(cols-3) {
            // let candidate = line[i_col..i_col+4].to_string();
            let candidate = &line[i_col..i_col+4];
            candidates.push(candidate)
        }
    }

    candidates
}

fn diagonal_candidates_down_right(input: &str) -> Vec<String> {
    let rows = num_rows(input);
    let cols = num_cols(input);

    let mut candidates: Vec<String> = vec![];

    let lines: Vec<&str> = input.lines().collect();

    for i_col in 0..cols-3 {
        for i_start_row in 0..rows-3 {
            let mut candidate = String::new();
            for j in 0..4 {
                let character = lines[i_start_row + j].chars().nth(i_col + j).unwrap();
                candidate.push(character);
            }
            candidates.push(candidate)
        }
    }

    candidates
}

fn diagonal_candidates_down_left(input: &str) -> Vec<String> {
    let rows = num_rows(input);
    let cols = num_cols(input);

    let mut candidates: Vec<String> = vec![];

    let lines: Vec<&str> = input.lines().collect();

    for i_col in 3..cols {
        for i_start_row in 0..rows-3 {
            let mut candidate = String::new();
            for j in 0..4 {
                let character = lines[i_start_row + j].chars().nth(i_col - j).unwrap();
                candidate.push(character);
            }
            candidates.push(candidate)
        }
    }

    candidates
}

fn x_candidates(input: &str) -> Vec<(String, String)> {
    let rows = num_rows(input);
    let cols = num_cols(input);

    let mut candidates: Vec<(String, String)> = vec![];

    let lines: Vec<&str> = input.lines().collect();

    for i_col in 0..cols-2 {
        for i_start_row in 0..rows-2 {
            let mut candidate1 = String::new();
            let mut candidate2 = String::new();
            for j in 0..3 {
                let character1 = lines[i_start_row + j].chars().nth(i_col + j).unwrap();
                let character2 = lines[i_start_row + j].chars().nth(i_col + 2 - j).unwrap();
                candidate1.push(character1);
                candidate2.push(character2);
            }
            let tup = (candidate1, candidate2);
            candidates.push(tup);
        }
    }

    candidates
}

pub fn is_xmas(s: &str) -> bool {
    s == "XMAS" || s == "SAMX"
}

pub fn is_mas(s: &str) -> bool {
    s == "MAS" || s == "SAM"
}

pub fn part_one(input: &str) -> Option<u32> {
    let num_horizontal = horizontal_candidates(input).into_iter().filter(|candidate| is_xmas(candidate)).count();
    let num_vertical = vertical_candidates(input).into_iter().filter(|candidate| is_xmas(candidate)).count();
    let num_diagonal_1 = diagonal_candidates_down_right(input).into_iter().filter(|candidate| is_xmas(candidate)).count();
    let num_diagonal_2 = diagonal_candidates_down_left(input).into_iter().filter(|candidate| is_xmas(candidate)).count();

    let sum: Option<u32> = (num_horizontal + num_vertical + num_diagonal_1 + num_diagonal_2).try_into().ok();
    sum
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum_cross = x_candidates(input).into_iter().filter(|(first, second)| is_mas(first) && is_mas(second)).count();
    let sum: Option<u32> = sum_cross.try_into().ok();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_candidates() {
        let result = horizontal_candidates("ASDFG");
        assert_eq!(result, vec!["ASDF", "SDFG"]);
    }


    #[test]
    fn test_vertical_candidates() {
        let result = vertical_candidates("A\nS\nD\nF\nG");
        assert_eq!(result, vec!["ASDF", "SDFG"]);
    }

    #[test]
    fn test_diagonal_down_right_candidates() {
        let result = diagonal_candidates_down_right("QWERT\nASDFG\nZXCVB\nYUIOP\nHJKNM");
        assert_eq!(result, vec!["QSCO", "AXIN", "WDVP", "SCOM"]);
    }

    #[test]
    fn test_diagonal_down_left_candidates() {
        let result = diagonal_candidates_down_left("QWERT\nASDFG\nZXCVB\nYUIOP\nHJKNM");
        assert_eq!(result, vec!["RDXY", "FCUH", "TFCU", "GVIJ"]);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
