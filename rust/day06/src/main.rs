use regex::Regex;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Race {
    distance: i64,
    time: i64,
}

fn main() {
    let file: String = fs::read_to_string("in.txt").expect("can't read file");

    let data_part_one = parse_data_part_one(&file);
    let data_part_two = parse_data_part_two(&file);

    println!("{:?}", data_part_two);

    println!("Solution Part 1: {}", part_one(&data_part_one));
    println!("Solution Part 2: {}", part_two(data_part_two));
}

fn part_one(races: &Vec<Race>) -> i64 {
    let mut acc: i64 = 1;

    for race in races.iter() {
        let time = race.time;
        for t in 0..=time {
            let distance = (time - t) * t;
            if distance > race.distance {
                acc *= (race.time - t) - t + 1;
                break;
            }
        }
    }
    acc
}

fn part_two(race: Race) -> i64 {
    let time = race.time;
    for t in 0..=time {
        let distance = (time - t) * t;
        if distance > race.distance {
            return (race.time - t) - t + 1;
        }
    }
    0
}

fn parse_data_part_one(data: &str) -> Vec<Race> {
    let num_regex: Regex = Regex::new(r"\s+").unwrap();

    let matrix: Vec<Vec<i64>> = data
        .lines()
        .map(|line| {
            let numbers_string = line.split(':').collect::<Vec<&str>>()[1].trim();

            let numbers = num_regex
                .split(numbers_string)
                .map(|number_string| number_string.parse().unwrap())
                .collect::<Vec<i64>>();

            numbers
        })
        .collect();

    let mut transposed: Vec<Race> = Vec::new();

    for i in 0..matrix[0].len() {
        transposed.push(Race {
            distance: matrix[1][i],
            time: matrix[0][i],
        })
    }

    transposed
}

fn parse_data_part_two(data: &str) -> Race {
    let num_regex: Regex = Regex::new(r"\s+").unwrap();

    let race_vec: Vec<i64> = data
        .lines()
        .map(|line| {
            let numbers_string = line.split(':').collect::<Vec<&str>>()[1].trim();

            let numbers = num_regex.replace_all(numbers_string, "").parse().unwrap();

            numbers
        })
        .collect();

    Race {
        distance: race_vec[1],
        time: race_vec[0],
    }
}
