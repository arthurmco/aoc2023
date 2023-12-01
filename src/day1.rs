// https://adventofcode.com/2023/day/1

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_file_as_text(path: &str) -> BufReader<File> {
    let f = File::open(path).unwrap();
    BufReader::new(f)
}

fn fix_incorrect_line(line: &str) -> u64 {
    let first = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last = line.chars().rfind(|c| c.is_digit(10)).unwrap();

    format!("{}{}", first, last).parse::<u64>().unwrap()
}

pub fn day1() {
    let coord_file = read_file_as_text("./inputs/day1real.txt");
    
    println!("{}", coord_file.lines().map(|l| fix_incorrect_line(&l.unwrap())).sum::<u64>())
}
