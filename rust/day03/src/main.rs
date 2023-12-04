use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

static ADJACENT_MOVES: [(isize, isize); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn main() {
    let line_matrix: Vec<Vec<char>> = read_lines("in.txt")
        .expect("can't read file")
        .flatten()
        .map(|line| line.chars().collect())
        .collect();

    println!("Solution Part 1: {}", part_one(&line_matrix));
    println!("Solution Part 2: {}", part_two(&line_matrix));
}

fn part_one(matrix: &[Vec<char>]) -> i32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut total = 0;

    for (line_index, line) in matrix.iter().enumerate() {
        for (col_index, char) in line.iter().enumerate() {
            if !char.is_numeric() && *char != '.' {
                total += check_adjacent(matrix, &mut visited, line_index, col_index)
                    .iter()
                    .sum::<i32>();
            }
        }
    }
    total
}

fn part_two(matrix: &[Vec<char>]) -> i32 {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut total = 0;

    for (line_index, line) in matrix.iter().enumerate() {
        for (col_index, char) in line.iter().enumerate() {
            if *char == '*' {
                let adjacent_numbers = check_adjacent(matrix, &mut visited, line_index, col_index);
                if adjacent_numbers.len() == 2 {
                    total += adjacent_numbers.iter().product::<i32>()
                }
            }
        }
    }
    total
}

fn check_adjacent(
    matrix: &[Vec<char>],
    visited: &mut HashSet<(usize, usize)>,
    line: usize,
    col: usize,
) -> Vec<i32> {
    let mut adjacent_numbers: Vec<i32> = Vec::new();

    for (line_move, col_move) in ADJACENT_MOVES.iter() {
        let new_line_signed = line as isize + line_move;
        let new_col_signed = col as isize + col_move;

        if new_line_signed < 0
            || new_line_signed > (matrix.len() - 1) as isize
            || new_col_signed < 0
            || new_col_signed > (matrix[line].len() - 1) as isize
        {
            continue;
        }

        let new_line = new_line_signed as usize;
        let new_col = new_col_signed as usize;

        if matrix[new_line][new_col].is_numeric() {
            let mut number_start_index = new_col;
            let mut number_end_index = new_col;

            for i in (0..new_col).rev() {
                if !matrix[new_line][i].is_numeric() {
                    break;
                }
                number_start_index = i
            }

            for i in new_col..matrix[new_line].len() {
                if !matrix[new_line][i].is_numeric() {
                    break;
                }
                number_end_index = i
            }

            if visited.contains(&(new_line, number_start_index)) {
                continue;
            }

            let num_string = matrix[new_line][number_start_index..=number_end_index]
                .iter()
                .collect::<String>();

            let num = num_string.parse::<i32>().expect("not a number");

            visited.insert((new_line, number_start_index));

            adjacent_numbers.push(num);
        }
    }

    adjacent_numbers
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
