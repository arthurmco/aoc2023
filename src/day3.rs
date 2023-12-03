use crate::util::read_file_as_text;
use regex::Regex;
use std::io::prelude::*;

#[derive(Debug)]
struct Schematic {
    data: Vec<Vec<char>>,
}

type SchematicNumber = (usize, usize, usize, u32);

impl Schematic {
    fn from_line_iter<S: Iterator<Item = String>>(iter: S) -> Schematic {
        Schematic {
            data: iter.map(|c| c.chars().collect()).collect(),
        }
    }

    fn retrieve_numbers_and_positions(&self) -> Vec<SchematicNumber> {
        let regex = Regex::new(r"(\d+)").unwrap();

        self.data
            .iter()
            .enumerate()
            .map(|(lno, cline)| (lno, cline.iter().clone().collect::<String>()))
            .flat_map(|(lno, line)| {
                //eprintln!("{:?}", line);
                regex
                    .find_iter(&line)
                    .map(move |m| (lno, m.start(), m.end(), m.as_str().parse::<u32>().unwrap()))
                    .collect::<Vec<SchematicNumber>>()
            })
            .collect()
    }

    fn is_symbol(value: char) -> bool {
        !value.is_digit(10) && value != '.'
    }

    fn is_adjacent_to_symbol(&self, x: usize, y: usize) -> bool {
        let ix = x as isize;
        let iy = y as isize;
        
        [
            (iy - 1, ix - 1),
            (iy - 1, ix),
            (iy - 1, ix + 1),
            (iy, ix - 1),
            (iy, ix + 1),
            (iy + 1, ix - 1),
            (iy + 1, ix),
            (iy + 1, ix + 1),
        ].into_iter().filter(|&(ay, ax)| {
            ay >= 0 && ax >= 0 && ay < (self.data.len() as isize) && ax < (self.data[0].len() as isize)
        }).any(|(ay, ax)| {
            let c = self.data[ay as usize][ax as usize];
            Schematic::is_symbol(c)
        })
    }

    fn retrieve_only_part_numbers(&self, nums: Vec<SchematicNumber>) -> Vec<u32> {
        nums.iter().filter(|&&(lineno, xstart, xend, _)| {
            (xstart..xend).any(|xval| self.is_adjacent_to_symbol(xval, lineno))
        }).map(|&(_, _, _, number)| number).collect()
    }
    
}

pub fn day3() {
    let game_file = read_file_as_text("./inputs/day3real.txt");
    //let game_file = read_file_as_text("./inputs/day3test.txt");

    let schematic =
        Schematic::from_line_iter(game_file.lines().filter(|s| s.is_ok()).map(|s| s.unwrap()));

    let numbers = schematic.retrieve_numbers_and_positions();
    //println!("Hello, {:?}", schematic);
    //println!("Hello, {:?}", numbers);
    let part_numbers = schematic.retrieve_only_part_numbers(numbers);
    //println!("Hello, {:?}", part_numbers);

    println!("{}", part_numbers.into_iter().map(|v| v as u64).sum::<u64>())
}
