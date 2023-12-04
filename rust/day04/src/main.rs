use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let line_matrix: Vec<String> = read_lines("in.txt")
        .expect("can't read file")
        .flatten()
        .collect();

    let mut total_part_one = 0;

    let mut instances: HashMap<usize, i32> = HashMap::new();

    for (index, line) in line_matrix.iter().enumerate() {
        let (winning_numbers, my_numbers) = process_line(&line);
        total_part_one += part_one(&winning_numbers, &my_numbers);
        part_two(
            &winning_numbers,
            &my_numbers,
            &mut instances,
            index,
            line_matrix.len(),
        );
    }

    println!("Solution Part 1: {}", total_part_one);
    println!("Solution Part 2: {}", instances.values().sum::<i32>());
}

fn part_one(winning_numbers: &HashSet<i32>, my_numbers: &Vec<i32>) -> i32 {
    my_numbers.iter().fold(0, |acc, num| {
        if !winning_numbers.contains(num) {
            return acc;
        }

        if acc == 0 {
            1
        } else {
            acc * 2
        }
    })
}

fn part_two(
    winning_numbers: &HashSet<i32>,
    my_numbers: &Vec<i32>,
    instances: &mut HashMap<usize, i32>,
    current_line: usize,
    total_cards: usize,
) {
    if instances.get(&current_line).is_none() {
        instances.insert(current_line, 1);
    }

    let total_won: usize = my_numbers.iter().fold(0, |acc, num| {
        if !winning_numbers.contains(num) {
            return acc;
        }

        acc + 1
    });

    for index in current_line + 1..=current_line + total_won {
        if index >= total_cards {
            break;
        }
        if instances.get(&index).is_none() {
            instances.insert(index, 1 + instances[&current_line]);
        } else {
            instances.insert(index, instances[&index] + instances[&current_line]);
        }
    }
}

fn process_line(line: &str) -> (HashSet<i32>, Vec<i32>) {
    let split: Vec<&str> = line.split('|').collect();

    let winning_numbers: HashSet<i32> = split[0].split(':').collect::<Vec<&str>>()[1]
        .trim()
        .split_whitespace()
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    let my_numbers: Vec<i32> = split[1]
        .trim()
        .split_whitespace()
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    (winning_numbers, my_numbers)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
