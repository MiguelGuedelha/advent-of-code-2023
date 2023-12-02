use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

static MAX_GREEN: i32 = 13;
static MAX_BLUE: i32 = 14;
static MAX_RED: i32 = 12;

fn main() {
    let lines = read_lines("in.txt");

    if let Ok(lines) = lines {
        let mut total_part_one: i32 = 0;
        let mut total_part_two: i32 = 0;

        for line in lines.flatten() {
            let (game_id, games) = process_line(&line);
            total_part_one += part_one(game_id, &games);
            total_part_two += part_two(&games);
        }

        println!("Solution Part 1: {}", total_part_one);
        println!("Solution Part 2: {}", total_part_two);
    }
}

fn part_one(game_id: i32, games: &Vec<Vec<(&str, i32)>>) -> i32 {
    for set in games {
        for (color, cubes_count) in set {
            let count = *cubes_count;
            match *color {
                "green" => {
                    if count > MAX_GREEN {
                        return 0;
                    };
                }
                "red" => {
                    if count > MAX_RED {
                        return 0;
                    };
                }
                "blue" => {
                    if count > MAX_BLUE {
                        return 0;
                    };
                }
                _ => (),
            }
        }
    }

    game_id
}

fn part_two(games: &Vec<Vec<(&str, i32)>>) -> i32 {
    let mut min_green = 0;
    let mut min_blue = 0;
    let mut min_red = 0;

    for set in games {
        for (color, cubes_count) in set {
            let count = *cubes_count;
            match *color {
                "green" => {
                    if count > min_green {
                        min_green = *cubes_count
                    }
                }
                "red" => {
                    if count > min_red {
                        min_red = *cubes_count
                    }
                }
                "blue" => {
                    if count > min_blue {
                        min_blue = *cubes_count
                    }
                }
                _ => (),
            }
        }
    }

    min_blue * min_green * min_red
}

fn process_line(line: &str) -> (i32, Vec<Vec<(&str, i32)>>) {
    let split_one: Vec<&str> = line.split(':').collect();
    let game_id: i32 = split_one[0]
        .split(' ')
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse()
        .unwrap();

    let games: Vec<Vec<(&str, i32)>> = split_one[1]
        .split(';')
        .map(|game| {
            return game
                .split(", ")
                .map(|color| {
                    let number_color: Vec<&str> = color.trim().split(' ').collect();
                    (number_color[1], number_color[0].parse().unwrap())
                })
                .collect::<Vec<(&str, i32)>>();
        })
        .collect();

    (game_id, games)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
