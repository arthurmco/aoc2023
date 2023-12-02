// https://adventofcode.com/2023/day/1

use crate::util::read_file_as_text;
use std::io::prelude::*;

fn _fix_incorrect_line(line: &str) -> u64 {
    let first = line.chars().find(|c| c.is_digit(10)).unwrap();
    let last = line.chars().rfind(|c| c.is_digit(10)).unwrap();

    format!("{}{}", first, last).parse::<u64>().unwrap()
}

pub fn _day1t1() {
    let coord_file = read_file_as_text("./inputs/day1real.txt");
    
    println!("{}", coord_file.lines().map(|l| _fix_incorrect_line(&l.unwrap())).sum::<u64>())
}

fn try_convert_into_number(line_piece: &str) -> Option<(String, usize)> {
    // Numbers from 1 to 9, sorted by string size.
    // This is mostly for saving time...
    //
    // The objective is to parse 'eightwothree' as 8wo3 and not eigh23.
    //  
    [
        (3, "three"),
        (7, "seven"),
        (8, "eight"),
        (4, "four"),
        (5, "five"),
        (9, "nine"),
        (1, "one"),
        (2, "two"),
        (6, "six"),
    ].into_iter().find(
        |(_num, name)| line_piece.starts_with(name)
    ).map(|(num, name)| (num.to_string(), name.len()))
}

fn transform_line(line: &str) -> String {
    let linem = line.to_lowercase();

    let mut resline = String::new();
    let mut index = 0;

    while index < linem.len() {
        let skip = match try_convert_into_number(&linem[index..]) {
            Some((num, skip)) => {
                resline += &num;
                skip-1 // not really correct, but good enough
            }
            None => {
                resline += &String::from(linem.chars().nth(index).unwrap());
                1
            }
        };

        index += skip
    }
       
    resline
}


fn fix_incorrect_line2(line: &str) -> u64 {
    let tline = transform_line(line);
    let first = tline.chars().find(|c| c.is_digit(10)).unwrap();
    let last = tline.chars().rfind(|c| c.is_digit(10)).unwrap();

    // println!("{}{}", first, last);
    
    format!("{}{}", first, last).parse::<u64>().unwrap()
}


pub fn day1() {
    let coord_file = read_file_as_text("./inputs/day1real.txt");
    
    println!("{}", coord_file.lines().map(|l| fix_incorrect_line2(&l.unwrap())).sum::<u64>())
}
