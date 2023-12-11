use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines: Vec<String> = read_lines("in_real.txt")
        .expect("can't read file")
        .flatten()
        .collect();

    let instructions: &str = &lines[0];
    let map_lines: &[String] = &lines[2..];

    let mut letter_map: HashMap<&str, (String, String)> = HashMap::new();

    map_lines.iter().for_each(|line| {
        let split: Vec<&str> = line.split(" = ").collect();
        let origin = split[0];
        let destinations: Vec<&str> = split[1].split(", ").collect();

        let value = (
            destinations[0].replace('(', ""),
            destinations[1].replace(')', ""),
        );
        letter_map.insert(origin, value);
    });

    println!("Solution Part 1: {}", part_one(&letter_map, instructions));
    println!("Solution Part 2: {}", part_two(&letter_map, instructions));
}

fn part_one(map: &HashMap<&str, (String, String)>, instructions: &str) -> i64 {
    let mut count = 0;

    let mut current = "AAA";

    let moves: Vec<char> = instructions.chars().collect();

    let mut i = 0;

    loop {
        let map_move = moves[i];
        let moves = &map[current];
        match map_move {
            'L' => current = moves.0.as_str(),
            'R' => current = moves.1.as_str(),
            _ => panic!(),
        }

        count += 1;

        if current == "ZZZ" {
            return count;
        }

        i += 1;
        i %= instructions.len();
    }
}

fn part_two(map: &HashMap<&str, (String, String)>, instructions: &str) -> i64 {
    let mut count = 0;

    let mut current: Vec<&str> = map
        .keys()
        .cloned()
        .filter(|key| key.ends_with('A'))
        .collect();

    let mut individual_moves_to_reach_end: Vec<i64> = vec![0; current.len()];

    let moves: Vec<char> = instructions.chars().collect();

    let mut i = 0;

    loop {
        for j in 0..current.len() {
            let map_move = moves[i];
            let moves = &map[current[j]];
            match map_move {
                'L' => current[j] = moves.0.as_str(),
                'R' => current[j] = moves.1.as_str(),
                _ => panic!(),
            }

            if current[j].ends_with('Z') {
                individual_moves_to_reach_end[j] = count + 1;
            }
        }

        count += 1;

        if individual_moves_to_reach_end
            .iter()
            .all(|&move_count| move_count > 0)
        {
            println!("Found all min steps");
            break;
        }

        i += 1;
        i %= instructions.len();
    }

    lcm_of_multiple(&individual_moves_to_reach_end)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn lcm_of_multiple(numbers: &[i64]) -> i64 {
    if numbers.len() <= 2 {
        return lcm(numbers[0], numbers[1]);
    }
    lcm(numbers[0], lcm_of_multiple(&numbers[1..]))
}

fn lcm(num_one: i64, num_two: i64) -> i64 {
    println!("LCM of {} and {}", num_one, num_two);
    num_one * num_two / gcd(num_one, num_two)
}

fn gcd(num_one: i64, num_two: i64) -> i64 {
    if num_two == 0 {
        return num_one;
    }
    gcd(num_two, num_one % num_two)
}
