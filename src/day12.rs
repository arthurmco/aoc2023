use crate::util::{read_file_as_text, split_numbers_by_comma};
use itertools::Itertools;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Clone)]
enum Spring {
    Operational,
    Damaged,
}

impl Spring {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Spring::Operational),
            '#' => Some(Spring::Damaged),
            '?' => None,
            _ => panic!("Unknown char"),
        }
    }
}

type DamagedSpringCount = usize;

type SpringRow = (Vec<Option<Spring>>, Vec<DamagedSpringCount>);

#[derive(Debug)]
struct SpringMap {
    rows: Vec<SpringRow>,
}

impl SpringMap {
    fn parse_spring_line(line: &str) -> SpringRow {
        let mut parts = line.split(' ');

        let springs = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| Spring::from_char(c))
            .collect();
        let counts = split_numbers_by_comma(&parts.next().unwrap());

        (springs, counts)
    }

    fn new(lines: impl Iterator<Item = String>) -> Self {
        let rows = lines.map(|l| SpringMap::parse_spring_line(&l)).collect();

        SpringMap { rows }
    }

    fn verify_if_line_is_valid(&self, springs: &[Spring], counts: &[DamagedSpringCount]) -> bool {
        let groups = springs.into_iter().group_by(|&s| *s == Spring::Damaged);
        let mut citer = counts.iter();
        
        groups.into_iter()
            .filter_map(|(k, g)| match k {
                false => None,
                true => Some(g),
            })
            .all(|g| g.count() == *citer.next().unwrap_or(&0)) && citer.next().is_none()
    }

    fn match_empty_rows_with_vertices(row: &[Option<Spring>], slots: &[Spring]) -> Vec<Spring> {
        let mut idx = 0;
        row.iter()
            .map(|v| {
                let cpos = idx;
                if v.is_some() {
                    v.clone().unwrap()
                } else {
                    idx += 1;
                    slots[cpos].clone()
                }
            })
            .collect()
    }

    fn generate_permutation_for<'a>(
        row: &'a [Option<Spring>],
    ) -> impl Iterator<Item = Vec<Spring>> + 'a {
        let empty_slots = row.iter().filter(|r| r.is_none()).count();
        
        itertools::repeat_n([Spring::Operational, Spring::Damaged], empty_slots)
            .multi_cartesian_product()
            //.inspect(|p| println!("p! {:?}", p))
            .map(move |slots| SpringMap::match_empty_rows_with_vertices(&row, &slots))
    }

    fn generate_permutation_count_for(&self, index: usize) -> usize {
        let (spring, counts) = &self.rows[index];

        SpringMap::generate_permutation_for(spring)
            .filter(|p| self.verify_if_line_is_valid(p, counts))
            //.inspect(|p| println!("p {:?} {:?}", p, counts))
            .count()
    }

    fn generate_permutation_sum(&self) -> usize {
        (0..self.rows.len()).into_iter().map(|i| self.generate_permutation_count_for(i)).sum()
    }
}

pub fn day12() {
    //let game_file = read_file_as_text("./inputs/day12test.txt").lines();
    let game_file = read_file_as_text("./inputs/day12real.txt").lines();
    let sm = SpringMap::new(game_file.filter_map(|s| s.ok()));

    //println!("Hello {:?}", sm);
    println!("{}", sm.generate_permutation_sum());
}
