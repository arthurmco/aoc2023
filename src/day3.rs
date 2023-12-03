use crate::util::read_file_as_text;
use regex::Regex;
use std::io::prelude::*;

#[derive(Debug)]
struct Schematic {
    data: Vec<Vec<char>>,
}

type SchematicNumber = (usize, usize, usize, u32);
type GearPosition = (usize, usize);

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

    fn retrieve_possible_gears_position(&self) -> Vec<GearPosition> {
        let regex = Regex::new(r"\*").unwrap();

        self.data
            .iter()
            .enumerate()
            .map(|(lno, cline)| (lno, cline.iter().clone().collect::<String>()))
            .flat_map(|(lno, line)| {
                //eprintln!("{:?}", line);
                regex
                    .find_iter(&line)
                    .map(move |m| (lno, m.start()))
                    .collect::<Vec<GearPosition>>()
            })
            .collect()
    }

    fn is_symbol(value: char) -> bool {
        !value.is_digit(10) && value != '.'
    }

    fn generate_adjacencies(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
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
        ]
        .into_iter()
        .filter(|&(ay, ax)| {
            ay >= 0
                && ax >= 0
                && ay < (self.data.len() as isize)
                && ax < (self.data[0].len() as isize)
        })
        .map(|(ay, ax)| (ay as usize, ax as usize))
        .collect()
    }

    fn is_adjacent_to_symbol(&self, x: usize, y: usize) -> bool {
        Schematic::generate_adjacencies(&self, x, y)
            .into_iter()
            .any(|(ay, ax)| {
                let c = self.data[ay as usize][ax as usize];
                Schematic::is_symbol(c)
            })
    }

    fn retrieve_only_part_numbers(&self, nums: Vec<SchematicNumber>) -> Vec<SchematicNumber> {
        nums.iter()
            .filter(|&&(lineno, xstart, xend, _)| {
                (xstart..xend).any(|xval| self.is_adjacent_to_symbol(xval, lineno))
            })
            .cloned()
            .collect()
    }

    fn retrieve_part_numbers_for_each_possible_gear(
        &self,
        gear: GearPosition,
        nums: &[SchematicNumber],
    ) -> Vec<u32> {
        let (gy, gx) = gear;
        nums.iter()
            .filter(|&&(lineno, xstart, xend, _)| {
                Schematic::generate_adjacencies(&self, gx, gy)
                    .into_iter()
                    .any(|(ay, ax)| lineno == ay && xstart <= ax && ax < xend)
            })
            .map(|&(_, _, _, number)| number)
            .collect()
    }

    fn retrieve_part_numbers_for_all_gears(
        &self,
        gears: Vec<GearPosition>,
        nums: Vec<SchematicNumber>,
    ) -> Vec<(GearPosition, u32, u32)> {
        gears
            .iter()
            .map(|gear| {
                (
                    gear,
                    self.retrieve_part_numbers_for_each_possible_gear(*gear, &nums),
                )
            })
            .filter(|(_, nums)| nums.len() == 2)
            .map(|(gear, nums)| (*gear, nums[0], nums[1]))
            .collect()
    }
    
    fn retrieve_gear_ratio_sum(
        &self,
        gears: Vec<(GearPosition, u32, u32)>
    ) -> u64 {
        gears
            .iter()
            .map(|(_, g1, g2)| (g1*g2) as u64)
            .sum()
    }
}

pub fn day3t1() {
    let game_file = read_file_as_text("./inputs/day3real.txt");
    //let game_file = read_file_as_text("./inputs/day3test.txt");

    let schematic =
        Schematic::from_line_iter(game_file.lines().filter(|s| s.is_ok()).map(|s| s.unwrap()));

    let numbers = schematic.retrieve_numbers_and_positions();
    //println!("Hello, {:?}", schematic);
    //println!("Hello, {:?}", numbers);
    let part_numbers: Vec<u32> = schematic
        .retrieve_only_part_numbers(numbers)
        .into_iter()
        .map(|(_, _, _, number)| number)
        .collect();
    //println!("Hello, {:?}", part_numbers);

    println!(
        "{}",
        part_numbers.into_iter().map(|v| v as u64).sum::<u64>()
    )
}

pub fn day3() {
    let game_file = read_file_as_text("./inputs/day3real.txt");
    //let game_file = read_file_as_text("./inputs/day3test.txt");

    let schematic =
        Schematic::from_line_iter(game_file.lines().filter(|s| s.is_ok()).map(|s| s.unwrap()));

    let numbers = schematic.retrieve_numbers_and_positions();
    let gears = schematic.retrieve_possible_gears_position();
    println!("Hello, {:?}", gears);
    let gears = schematic.retrieve_part_numbers_for_all_gears(gears, numbers);
    println!("Hello, {:?}", gears);

    let sum = schematic.retrieve_gear_ratio_sum(gears);
    println!("{}", sum);
        

    //println!("{}", part_numbers.into_iter().map(|v| v as u64).sum::<u64>())
}
