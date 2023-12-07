use regex::Regex;
use std::fs;

fn main() {
    let file: String = fs::read_to_string("in.txt").expect("can't read file");

    let (seeds, maps) = parse_data(&file);

    println!("Solution Part 1: {}", part_one(&seeds, &maps));
    println!("Solution Part 2: {}", part_two(&seeds, &maps));
}

fn part_one(seeds: &[i64], maps: &Vec<Vec<((i64, i64), (i64, i64))>>) -> i64 {
    let mut min_location = std::i64::MAX;

    for seed in seeds {
        let mut current = *seed;
        for map in maps.iter() {
            for entry in map {
                if current >= entry.1 .0 && current <= entry.1 .1 {
                    current = entry.0 .0 + (current - entry.1 .0);
                    break;
                }
            }
        }
        if current < min_location {
            min_location = current
        }
    }

    min_location
}

fn part_two(seeds: &[i64], maps: &Vec<Vec<((i64, i64), (i64, i64))>>) -> i64 {
    let mut min_location = std::i64::MAX;

    let seed_ranges: Vec<(i64, i64)> = seeds
        .chunks_exact(2)
        .map(|vec| (vec[0], vec[0] + vec[1] - 1))
        .collect();

    for (start, finish) in seed_ranges {
        for seed in start..=finish {
            let mut current = seed;
            for map in maps.iter() {
                for entry in map {
                    if current >= entry.1 .0 && current <= entry.1 .1 {
                        current = entry.0 .0 + (current - entry.1 .0);
                        break;
                    }
                }
            }
            if current < min_location {
                min_location = current
            }
        }
    }

    min_location
}

fn parse_data(data: &str) -> (Vec<i64>, Vec<Vec<((i64, i64), (i64, i64))>>) {
    let double_new_line_regex = Regex::new(r"\r?\n\r?\n").unwrap();
    let num_regex = Regex::new(r"\d+").unwrap();

    let seeds_and_maps: Vec<Vec<Vec<i64>>> = double_new_line_regex
        .split(data)
        .map(|group| {
            group
                .lines()
                .filter_map(|line| {
                    let numbers: Vec<i64> = num_regex
                        .find_iter(line)
                        .flat_map(|m| m.as_str().parse::<i64>().ok())
                        .collect();

                    // Return the numbers for the current line, or None if no numbers are found
                    if !numbers.is_empty() {
                        Some(numbers)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let seeds = seeds_and_maps[0][0].clone();
    let maps = &seeds_and_maps[1..];

    let mut range_maps: Vec<Vec<((i64, i64), (i64, i64))>> = maps
        .iter()
        .map(|chunk| {
            chunk
                .iter()
                .map(|numbers| {
                    let dest_start = numbers[0];
                    let source_start = numbers[1];
                    let span = numbers[2];

                    (
                        (dest_start, dest_start + span - 1),
                        (source_start, source_start + span - 1),
                    )
                })
                .collect()
        })
        .collect();

    range_maps
        .iter_mut()
        .for_each(|vec| vec.sort_by_key(|x| x.1 .0));

    (seeds, range_maps)
}
