use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn main() {
    let lines: Vec<String> = read_lines("in_real.txt")
        .expect("can't read file")
        .flatten()
        .collect();

    let number_lines: Vec<Vec<i64>> = lines
        .iter()
        .map(|line| line.split(' ').map(|num| num.parse().unwrap()).collect())
        .collect();

    println!("Solution Part 1: {}", part_one(&number_lines));
    println!("Solution Part 2: {}", part_two(&number_lines));
}

fn part_one(lines: &Vec<Vec<i64>>) -> i64 {
    for i in 0..lines.len() {
        let mut extrapolation_matrix: Vec<Vec<i64>> = Vec::new();
        let line = &lines[i];
        extrapolation_matrix.push(line.clone());

        loop {
            let last = extrapolation_matrix.last().unwrap();
            let mut next_line: Vec<i64> = Vec::new();
            let mut all_zero = true;
            for z in 0..last.len() - 1 {
                let result = last[z + 1] - last[z];
                next_line.push(last[z + 1] - last[z]);
                if result != 0 {
                    all_zero = false;
                }

                if z == last.len() - 2 && all_zero {
                    next_line.push(0);
                }
            }

            extrapolation_matrix.push(next_line);

            if all_zero {
                break;
            }
        }
    }
    0
}

fn part_two(lines: &Vec<Vec<i64>>) -> i64 {
    0
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
