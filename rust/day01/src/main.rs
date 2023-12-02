use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("in.txt") {
        let mut total_part_one: i32 = 0;
        let mut total_part_two: i32 = 0;

        let digits = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ];

        for line in lines {
            if let Ok(ip) = line {
                total_part_one += part_one(&ip);
                total_part_two += part_two(&ip, &digits);
            }
        }

        println!("Total Part 1: {}", total_part_one);
        println!("Total Part 2: {}", total_part_two);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one(line: &String) -> i32 {
    let mut first_digit: char = 'a';
    let mut last_digit: char = 'a';
    let char_vec: Vec<char> = line.chars().collect();
    for character in char_vec {
        if character.is_numeric() && first_digit == 'a' {
            first_digit = character
        }
        if character.is_numeric() && first_digit != 'a' {
            last_digit = character
        }
    }

    let line_number_string: String = format!("{}{}", first_digit, last_digit);
    return line_number_string.parse::<i32>().unwrap();
}

fn part_two(line: &String, digit_names: &Vec<&str>) -> i32 {
    let mut indices_map: Vec<(usize, &str)> = Vec::new();

    for digit in digit_names.iter() {
        let indices: Vec<_> = line.match_indices(digit).collect();
        if indices.len() > 0 {
            indices_map.push(indices.first().unwrap().clone());
            indices_map.push(indices.last().unwrap().clone());
        }
    }

    indices_map.sort_by(|a, b| a.0.cmp(&b.0));
    let first_digit_tuple: &(usize, &str) = indices_map.first().unwrap();
    let last_digit_tuple: &(usize, &str) = indices_map.last().unwrap();

    let first_digit_og_index = digit_names
        .iter()
        .position(|&r| r == first_digit_tuple.1)
        .unwrap();
    let last_digit_og_index = digit_names
        .iter()
        .position(|&r| r == last_digit_tuple.1)
        .unwrap();

    let first_digit = if first_digit_og_index < 9 {
        first_digit_og_index + 1
    } else {
        first_digit_og_index - 8
    };

    let last_digit = if last_digit_og_index < 9 {
        last_digit_og_index + 1
    } else {
        last_digit_og_index - 8
    };

    let line_number_string: String = format!("{}{}", first_digit, last_digit);
    return line_number_string.parse::<i32>().unwrap();
}
