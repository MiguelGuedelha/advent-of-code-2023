use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let lines: Vec<String> = read_lines("in.txt")
        .expect("can't read file")
        .flatten()
        .collect();

    let string_maps: Vec<Vec<&str>> = lines
        .join("")
        .split("\n\n")
        .map(|line_str| line_str.lines().collect())
        .collect();

    let mut total_part_one = 0;
    let mut total_part_two = 0;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
